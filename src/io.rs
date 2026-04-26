use image;
use std::fmt;

pub struct RawImage {
    pub data: Vec<f32>,
    pub width: u32,
    pub height: u32,
    pub channels: usize,
}

impl fmt::Display for RawImage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "RawImage {{ data: {:?}, width: {}, height: {}, channels: {} }}",
            self.data, self.width, self.height, self.channels
        )
    }
}

pub fn load_image_raw(path: &str) -> Result<RawImage, String> {
    let img = image::open(path).map_err(|e| format!("Failed to load image: {}", e))?;
    let img = img.to_rgb8();
    let (w, h) = img.dimensions();
    let channels = 3;
    let data = img.into_raw().iter().map(|&x| x as f32 / 255.0).collect();
    return Ok(RawImage {
        data,
        width: w,
        height: h,
        channels,
    });
}

pub fn save_image_raw(path: &str, img: &RawImage) -> Result<(), String> {
    let u8_data = img
        .data
        .iter()
        .map(|&x| (x * 255.0) as u8)
        .collect::<Vec<u8>>();

    if img.channels == 1 {
        image::save_buffer(path, &u8_data, img.width, img.height, image::ColorType::L8)
    } else if img.channels == 3 {
        image::save_buffer(
            path,
            &u8_data,
            img.width,
            img.height,
            image::ColorType::Rgb8,
        )
    } else {
        return Err(format!("Unsupported number of channels: {}", img.channels));
    }
    .map_err(|e| format!("Failed to save image: {}", e))
}
