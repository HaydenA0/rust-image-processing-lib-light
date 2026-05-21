# Web Interface — How to Run

```bash
# From the workspace root
cargo run -p image_web
```

Open [http://localhost:8080](http://localhost:8080) in your browser.

## Usage

1. **Upload** — click the dashed area or drag & drop an image (PNG/JPG/WEBP)
2. **Process** — use the toolbar buttons:
   - `Grayscale`, `Rotate 90°/180°/270°`
   - `Blur ▸` to reveal sigma/kernel sliders, then `Apply Blur`
   - `Info` — shows mean, median, stddev, min/max
   - `Histogram` — luminance distribution bar chart
3. **Download** — saves the current processed image
