use axum::{
    extract::{Multipart, Query, State},
    http::StatusCode,
    response::{Html, IntoResponse, Json},
    routing::{get, post},
    Router,
};
use image_core::{
    getinfo,
    includes::RawImage,
    kernel,
    transformcolor,
};
use serde::Serialize;
use std::sync::{Arc, Mutex};

const MAX_HISTORY: usize = 20;

struct AppState {
    image: Mutex<Option<RawImage>>,
    history: Mutex<Vec<RawImage>>,
}

fn push_history(img: &mut Option<RawImage>, history: &mut Vec<RawImage>) {
    if let Some(ref current) = img {
        history.push(current.clone());
        if history.len() > MAX_HISTORY {
            history.remove(0);
        }
    }
}

#[derive(serde::Deserialize)]
struct RotateQuery {
    angle: u32,
}

#[derive(serde::Deserialize)]
struct BlurQuery {
    sigma: f32,
    size: usize,
}

#[derive(Serialize)]
struct InfoResponse {
    mean: f32,
    median: f32,
    max: f32,
    min: f32,
    stddev: f32,
    width: f32,
    height: f32,
}

#[derive(Serialize)]
struct MetadataResponse {
    width: u32,
    height: u32,
    channels: usize,
    pixels: usize,
}

fn load_image_from_bytes(data: &[u8]) -> Result<RawImage, String> {
    let img = image::load_from_memory(data).map_err(|e| format!("Failed to load: {}", e))?;
    let rgb = img.to_rgb8();
    let (w, h) = rgb.dimensions();
    let pixels = rgb.into_raw().iter().map(|&x| x as f32 / 255.0).collect();
    Ok(RawImage { data: pixels, x_size: w, y_size: h, channels: 3 })
}

fn save_image_to_png(img: &RawImage) -> Result<Vec<u8>, String> {
    let u8_data: Vec<u8> = img.data.iter().map(|&x| (x * 255.0) as u8).collect();
    let mut buf = std::io::Cursor::new(Vec::new());
    if img.channels == 1 {
        image::write_buffer_with_format(
            &mut buf, &u8_data, img.x_size, img.y_size,
            image::ColorType::L8, image::ImageFormat::Png,
        )
    } else if img.channels == 3 {
        image::write_buffer_with_format(
            &mut buf, &u8_data, img.x_size, img.y_size,
            image::ColorType::Rgb8, image::ImageFormat::Png,
        )
    } else {
        return Err(format!("Unsupported channels: {}", img.channels));
    }
    .map_err(|e| format!("Failed to encode: {}", e))?;
    Ok(buf.into_inner())
}

async fn index() -> Html<&'static str> {
    Html(HTML)
}

async fn upload(
    State(state): State<Arc<AppState>>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    while let Some(field) = multipart.next_field().await.map_err(|e| {
        (StatusCode::BAD_REQUEST, format!("Multipart error: {}", e))
    })? {
        if field.name() == Some("file") {
            let data = field.bytes().await.map_err(|e| {
                (StatusCode::BAD_REQUEST, format!("Read error: {}", e))
            })?;
            let raw = load_image_from_bytes(&data)
                .map_err(|e| (StatusCode::BAD_REQUEST, e))?;
            let mut img = state.image.lock().unwrap();
            let mut history = state.history.lock().unwrap();
            *img = Some(raw);
            history.clear();
            return Ok((StatusCode::OK, "OK"));
        }
    }
    Err((StatusCode::BAD_REQUEST, "No file field".into()))
}

async fn get_image(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let img = state.image.lock().unwrap();
    let img = img.as_ref().ok_or((StatusCode::NOT_FOUND, "No image".into()))?;
    let png = save_image_to_png(img)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;
    Ok((
        [("Content-Type", "image/png")],
        png,
    ))
}

async fn grayscale(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut img = state.image.lock().unwrap();
    let mut history = state.history.lock().unwrap();
    let channels = img.as_ref().map(|i| i.channels).ok_or((StatusCode::NOT_FOUND, "No image".into()))?;
    if channels == 1 {
        return Ok((StatusCode::OK, "Already grayscale"));
    }
    let current = img.clone().unwrap();
    push_history(&mut img, &mut history);
    let mut clone = current;
    let gray = transformcolor::make_grasyscale(&mut clone);
    *img = Some(gray);
    Ok((StatusCode::OK, "Grayscale applied"))
}

async fn rotate(
    State(state): State<Arc<AppState>>,
    query: Query<RotateQuery>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut img = state.image.lock().unwrap();
    let mut history = state.history.lock().unwrap();
    let current = img.as_ref().map(|i| i.clone()).ok_or((StatusCode::NOT_FOUND, "No image".into()))?;
    let rotated = transformcolor::rotate_image(&current, query.angle)
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;
    push_history(&mut img, &mut history);
    *img = Some(rotated);
    Ok((StatusCode::OK, format!("Rotated {}°", query.angle)))
}

async fn blur(
    State(state): State<Arc<AppState>>,
    query: Query<BlurQuery>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut img = state.image.lock().unwrap();
    let mut history = state.history.lock().unwrap();
    let current = img.as_ref().map(|i| i.clone()).ok_or((StatusCode::NOT_FOUND, "No image".into()))?;
    let blurred = kernel::apply_gaussian_blur(&current, query.sigma, query.size);
    push_history(&mut img, &mut history);
    *img = Some(blurred);
    Ok((StatusCode::OK, "Blur applied"))
}

async fn info(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let img = state.image.lock().unwrap();
    let current = img.as_ref().ok_or((StatusCode::NOT_FOUND, "No image".into()))?;
    let gray = if current.channels == 3 {
        let mut c = current.clone();
        transformcolor::make_grasyscale(&mut c)
    } else {
        current.clone()
    };
    let (mean, median, max, min, stddev, width, height) = getinfo::get_image_info(&gray);
    Ok(Json(InfoResponse { mean, median, max, min, stddev, width, height }))
}

async fn metadata(
    State(state): State<Arc<AppState>>,
) -> Result<Json<MetadataResponse>, (StatusCode, String)> {
    let img = state.image.lock().unwrap();
    let current = img.as_ref().ok_or((StatusCode::NOT_FOUND, "No image".into()))?;
    Ok(Json(MetadataResponse {
        width: current.x_size,
        height: current.y_size,
        channels: current.channels,
        pixels: current.data.len(),
    }))
}

async fn undo(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut img = state.image.lock().unwrap();
    let mut history = state.history.lock().unwrap();
    let prev = history.pop()
        .ok_or_else(|| (StatusCode::BAD_REQUEST, "Nothing to undo".to_string()))?;
    *img = Some(prev);
    Ok((StatusCode::OK, "Undone".to_string()))
}

async fn can_undo(
    State(state): State<Arc<AppState>>,
) -> Json<bool> {
    let history = state.history.lock().unwrap();
    Json(!history.is_empty())
}

async fn histogram(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let img = state.image.lock().unwrap();
    let current = img.as_ref().ok_or((StatusCode::NOT_FOUND, "No image".into()))?;
    let gray = if current.channels == 3 {
        let mut c = current.clone();
        transformcolor::make_grasyscale(&mut c)
    } else {
        current.clone()
    };
    let hist = getinfo::get_image_histogram(&gray);
    let normalized = getinfo::get_image_histogram_normalized(&hist);
    let mut buckets: Vec<HashMapEntry> = normalized
        .iter()
        .map(|(&k, &v)| HashMapEntry { bucket: k, count: v })
        .collect();
    buckets.sort_by_key(|e| e.bucket);
    Ok(Json(buckets))
}

#[derive(Serialize)]
struct HashMapEntry {
    bucket: u8,
    count: f32,
}

#[tokio::main]
async fn main() {
    let state = Arc::new(AppState {
        image: Mutex::new(None),
        history: Mutex::new(Vec::new()),
    });

    let app = Router::new()
        .route("/", get(index))
        .route("/upload", post(upload))
        .route("/image", get(get_image))
        .route("/grayscale", post(grayscale))
        .route("/rotate", post(rotate))
        .route("/blur", post(blur))
        .route("/undo", post(undo))
        .route("/can-undo", get(can_undo))
        .route("/info", get(info))
        .route("/metadata", get(metadata))
        .route("/histogram", get(histogram))
        .layer(tower_http::cors::CorsLayer::permissive())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("Server running at http://localhost:8080");
    axum::serve(listener, app).await.unwrap();
}

const HTML: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>Image Processor</title>
<style>
  * { box-sizing: border-box; margin: 0; padding: 0; }
  body { font-family: system-ui, sans-serif; background: #111; color: #eee; min-height: 100vh; }
  h1 { text-align: center; padding: 24px 0; font-weight: 300; letter-spacing: 1px; }
  .layout { display: flex; gap: 20px; max-width: 1100px; margin: 0 auto; padding: 0 20px; }
  .main { flex: 1; min-width: 0; }
  .sidebar { width: 220px; flex-shrink: 0; display: none; }
  .sidebar.show { display: block; }
  .upload-area { border: 2px dashed #444; border-radius: 12px; padding: 40px; text-align: center; margin-bottom: 20px; cursor: pointer; transition: 0.2s; }
  .upload-area:hover, .upload-area.dragover { border-color: #6af; background: #1a1a2e; }
  .upload-area input { display: none; }
  .upload-area label { cursor: pointer; }
  .upload-area p { color: #888; margin-top: 8px; font-size: 14px; }
  .image-wrapper { background: #1a1a1a; border-radius: 12px; overflow: hidden; margin-bottom: 20px; display: none; }
  .image-wrapper.show { display: block; }
  .image-wrapper img { width: 100%; display: block; }
  .toolbar { display: flex; flex-wrap: wrap; gap: 8px; margin-bottom: 16px; }
  .toolbar button, .toolbar a { background: #2a2a3e; border: none; color: #eee; padding: 10px 18px; border-radius: 8px; cursor: pointer; font-size: 14px; transition: 0.15s; text-decoration: none; white-space: nowrap; }
  .toolbar button:hover, .toolbar a:hover { background: #3a3a5e; }
  .toolbar button:active { background: #5a5a8e; }
  .toolbar button.primary { background: #2d5a8e; }
  .toolbar button.primary:hover { background: #3a7abe; }
  .controls { display: none; flex-wrap: wrap; gap: 16px; margin-bottom: 16px; padding: 16px; background: #1a1a2e; border-radius: 8px; align-items: center; }
  .controls.show { display: flex; }
  .controls label { font-size: 13px; color: #aaa; }
  .controls input[type="range"] { width: 120px; }
  .controls .value { font-size: 13px; color: #6af; min-width: 30px; text-align: center; }
  .panel { background: #1a1a2e; border-radius: 8px; padding: 16px; margin-bottom: 16px; display: none; }
  .panel.show { display: block; }
  .panel h3 { font-weight: 400; margin-bottom: 8px; color: #8af; }
  .panel table { width: 100%; border-collapse: collapse; }
  .panel td { padding: 4px 8px; font-size: 14px; }
  .panel td:first-child { color: #888; }
  .panel td:last-child { text-align: right; font-family: monospace; }
  .histogram-bar { display: inline-block; background: #6af; margin-right: 1px; vertical-align: bottom; border-radius: 1px; }
  .histogram-container { display: flex; align-items: flex-end; height: 150px; padding: 8px 0; }
  .status { color: #8c8; font-size: 14px; margin-bottom: 12px; min-height: 20px; text-align: center; }
  .status.error { color: #c88; }
  .spinner { display: none; width: 20px; height: 20px; border: 2px solid #444; border-top-color: #6af; border-radius: 50%; animation: spin 0.6s linear infinite; margin: 0 auto; }
  .spinner.show { display: block; }
  @keyframes spin { to { transform: rotate(360deg); } }
  .meta-card { background: #1a1a2e; border-radius: 12px; padding: 16px; }
  .meta-card h3 { font-weight: 400; font-size: 14px; color: #8af; margin-bottom: 12px; text-transform: uppercase; letter-spacing: 1px; }
  .meta-row { display: flex; justify-content: space-between; padding: 6px 0; font-size: 14px; border-bottom: 1px solid #222; }
  .meta-row:last-child { border-bottom: none; }
  .meta-row .label { color: #888; }
  .meta-row .value { color: #eee; font-family: monospace; }
  @media (max-width: 800px) { .layout { flex-direction: column; } .sidebar { width: 100%; } }
</style>
</head>
<body>
<h1>Image Processor</h1>
<div class="layout">
  <div class="main">
    <div class="upload-area" id="dropZone">
      <input type="file" id="fileInput" accept="image/*">
      <label for="fileInput"><strong>Click to upload</strong> or drag & drop an image</label>
      <p>PNG, JPG, WEBP</p>
    </div>
    <div class="status" id="status"></div>
    <div class="spinner" id="spinner"></div>
    <div class="image-wrapper" id="imageWrapper">
      <img id="preview" alt="Preview">
    </div>
    <div class="toolbar" id="toolbar" style="display:none;">
      <button class="primary" data-op="grayscale">Grayscale</button>
      <button id="undoBtn" disabled style="opacity:0.4">Undo</button>
      <button data-op="rotate" data-angle="90">Rotate 90°</button>
      <button data-op="rotate" data-angle="180">Rotate 180°</button>
      <button data-op="rotate" data-angle="270">Rotate 270°</button>
      <button id="toggleBlur">Blur ▸</button>
      <button id="toggleInfo">Stats</button>
      <button id="toggleHistogram">Histogram</button>
      <a href="/image" download="image.png" id="downloadBtn">Download</a>
    </div>
    <div class="controls" id="blurControls">
      <label>Sigma: <span id="sigmaVal">3.0</span></label>
      <input type="range" id="sigma" min="0.5" max="10" step="0.5" value="3">
      <label>Kernel: <span id="sizeVal">7</span></label>
      <input type="range" id="ksize" min="3" max="31" step="2" value="7">
      <button class="primary" id="applyBlur">Apply Blur</button>
    </div>
    <div class="panel" id="infoPanel"><h3>Stats</h3><table id="infoTable"></table></div>
    <div class="panel" id="histogramPanel"><h3>Histogram</h3><div class="histogram-container" id="histogramContainer"></div></div>
  </div>
  <div class="sidebar" id="sidebar">
    <div class="meta-card">
      <h3>Image</h3>
      <div class="meta-row"><span class="label">Dimensions</span><span class="value" id="metaDims">—</span></div>
      <div class="meta-row"><span class="label">Width</span><span class="value" id="metaWidth">—</span></div>
      <div class="meta-row"><span class="label">Height</span><span class="value" id="metaHeight">—</span></div>
      <div class="meta-row"><span class="label">Channels</span><span class="value" id="metaChannels">—</span></div>
      <div class="meta-row"><span class="label">Pixels</span><span class="value" id="metaPixels">—</span></div>
    </div>
  </div>
</div>

<script>
const fileInput = document.getElementById('fileInput');
const dropZone = document.getElementById('dropZone');
const preview = document.getElementById('preview');
const imageWrapper = document.getElementById('imageWrapper');
const toolbar = document.getElementById('toolbar');
const status = document.getElementById('status');
const spinner = document.getElementById('spinner');
const downloadBtn = document.getElementById('downloadBtn');
const sidebar = document.getElementById('sidebar');
const undoBtn = document.getElementById('undoBtn');

function setStatus(msg, err) { status.textContent = msg; status.className = 'status' + (err ? ' error' : ''); }
function showSpinner(v) { spinner.className = 'spinner' + (v ? ' show' : ''); }

async function uploadFile(file) {
  const fd = new FormData();
  fd.append('file', file);
  showSpinner(true);
  setStatus('Uploading...');
  try {
    const res = await fetch('/upload', { method: 'POST', body: fd });
    if (!res.ok) { const msg = await res.text(); setStatus(msg, true); showSpinner(false); return; }
    await loadImage();
    setStatus('Image loaded');
  } catch(e) { setStatus('Upload error: ' + e.message, true); }
  showSpinner(false);
}

async function updateUndo() {
  try {
    const res = await fetch('/can-undo');
    const ok = await res.json();
    undoBtn.disabled = !ok;
    undoBtn.style.opacity = ok ? '1' : '0.4';
  } catch(_) { undoBtn.disabled = true; undoBtn.style.opacity = '0.4'; }
}

async function loadMeta() {
  try {
    const res = await fetch('/metadata');
    if (!res.ok) { sidebar.classList.remove('show'); return; }
    const d = await res.json();
    document.getElementById('metaDims').textContent = d.width + ' × ' + d.height;
    document.getElementById('metaWidth').textContent = d.width;
    document.getElementById('metaHeight').textContent = d.height;
    document.getElementById('metaChannels').textContent = d.channels;
    document.getElementById('metaPixels').textContent = d.pixels.toLocaleString();
    sidebar.classList.add('show');
  } catch(_) { sidebar.classList.remove('show'); }
}

async function loadImage() {
  const res = await fetch('/image?' + Date.now());
  if (!res.ok) { imageWrapper.classList.remove('show'); toolbar.style.display = 'none'; sidebar.classList.remove('show'); return; }
  const blob = await res.blob();
  preview.src = URL.createObjectURL(blob);
  imageWrapper.classList.add('show');
  toolbar.style.display = 'flex';
  downloadBtn.href = '/image?' + Date.now();
  await loadMeta();
  await updateUndo();
}

dropZone.addEventListener('click', () => fileInput.click());

fileInput.addEventListener('change', () => {
  if (fileInput.files.length) uploadFile(fileInput.files[0]);
});

dropZone.addEventListener('dragover', (e) => { e.preventDefault(); dropZone.classList.add('dragover'); });
dropZone.addEventListener('dragleave', () => { dropZone.classList.remove('dragover'); });
dropZone.addEventListener('drop', (e) => {
  e.preventDefault();
  dropZone.classList.remove('dragover');
  if (e.dataTransfer.files.length) uploadFile(e.dataTransfer.files[0]);
});

undoBtn.addEventListener('click', async () => {
  showSpinner(true);
  setStatus('Undoing...');
  try {
    const res = await fetch('/undo', { method: 'POST' });
    const text = await res.text();
    if (!res.ok) { setStatus(text, true); showSpinner(false); return; }
    await loadImage();
    setStatus(text);
  } catch(e) { setStatus('Error: ' + e.message, true); }
  showSpinner(false);
});

document.querySelectorAll('[data-op]').forEach(btn => {
  btn.addEventListener('click', async () => {
    const op = btn.dataset.op;
    let url = '/' + op;
    if (op === 'move') url += '?axis=' + btn.dataset.axis;
    showSpinner(true);
    setStatus('Applying ' + op + '...');
    try {
      const params = new URLSearchParams();
      if (btn.dataset.angle) params.set('angle', btn.dataset.angle);
      const qs = params.toString();
      const res = await fetch(url + (qs ? '?' + qs : ''), { method: 'POST' });
      const text = await res.text();
      if (!res.ok) { setStatus(text, true); showSpinner(false); return; }
      await loadImage();
      setStatus(text);
    } catch(e) { setStatus('Error: ' + e.message, true); }
    showSpinner(false);
  });
});

document.getElementById('toggleBlur').addEventListener('click', () => {
  document.getElementById('blurControls').classList.toggle('show');
});
document.getElementById('sigma').addEventListener('input', function() {
  document.getElementById('sigmaVal').textContent = parseFloat(this.value).toFixed(1);
});
document.getElementById('ksize').addEventListener('input', function() {
  document.getElementById('sizeVal').textContent = this.value;
});
document.getElementById('applyBlur').addEventListener('click', async () => {
  const sigma = document.getElementById('sigma').value;
  const size = document.getElementById('ksize').value;
  showSpinner(true);
  setStatus('Applying blur...');
  try {
    const res = await fetch('/blur?sigma=' + sigma + '&size=' + size, { method: 'POST' });
    const text = await res.text();
    if (!res.ok) { setStatus(text, true); showSpinner(false); return; }
    await loadImage();
    setStatus(text);
  } catch(e) { setStatus('Error: ' + e.message, true); }
  showSpinner(false);
});

document.getElementById('toggleInfo').addEventListener('click', async () => {
  const panel = document.getElementById('infoPanel');
  panel.classList.toggle('show');
  if (panel.classList.contains('show')) {
    try {
      const res = await fetch('/info');
      const data = await res.json();
      const tbody = document.getElementById('infoTable');
      tbody.innerHTML = Object.entries(data).map(([k,v]) => '<tr><td>' + k + '</td><td>' + v.toFixed(4) + '</td></tr>').join('');
    } catch(e) { setStatus('Info error', true); }
  }
});

document.getElementById('toggleHistogram').addEventListener('click', async () => {
  const panel = document.getElementById('histogramPanel');
  panel.classList.toggle('show');
  if (panel.classList.contains('show')) {
    try {
      const res = await fetch('/histogram');
      const data = await res.json();
      const maxCount = Math.max(...data.map(d => d.count));
      const container = document.getElementById('histogramContainer');
      container.innerHTML = data.map(d => '<div class="histogram-bar" style="height:' + (d.count/maxCount*150) + 'px;width:3px" title="' + d.bucket + ': ' + (d.count*100).toFixed(1) + '%"></div>').join('');
    } catch(e) { setStatus('Histogram error', true); }
  }
});
</script>
</body>
</html>"#;
