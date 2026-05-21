use image_core::includes::RawImage;

use std::env;
use std::fs;

pub fn load_image_raw(path: &str) -> Result<RawImage, String> {
    let img = match image::open(path) {
        Ok(img) => img,
        Err(e) => {
            let current_dir =
                env::current_dir().unwrap_or_else(|_| panic!("Failed to get current directory"));
            let dir_contents: Vec<_> = fs::read_dir(&current_dir)
                .unwrap_or_else(|_| panic!("Failed to read directory contents"))
                .map(|entry| entry.unwrap().file_name().to_string_lossy().to_string())
                .collect();

            return Err(format!(
                "Failed to load image: {}.\nCurrent directory: {:?}\nContents: {:?}",
                e, current_dir, dir_contents
            ));
        }
    };
    let img = img.to_rgb8();
    let (w, h) = img.dimensions();
    let channels = 3;
    let data: Vec<f32> = img.into_raw().iter().map(|&x| x as f32 / 255.0).collect();
    Ok(RawImage {
        data,
        x_size: w,
        y_size: h,
        channels,
    })
}

pub fn save_image_raw(path: &str, img: &RawImage) -> Result<(), String> {
    let u8_data = img
        .data
        .iter()
        .map(|&x| (x * 255.0) as u8)
        .collect::<Vec<u8>>();

    if img.channels == 1 {
        image::save_buffer(path, &u8_data, img.x_size, img.y_size, image::ColorType::L8)
    } else if img.channels == 3 {
        image::save_buffer(
            path,
            &u8_data,
            img.x_size,
            img.y_size,
            image::ColorType::Rgb8,
        )
    } else {
        return Err(format!("Unsupported number of channels: {}", img.channels));
    }
    .map_err(|e| format!("Failed to save image: {}", e))
}
