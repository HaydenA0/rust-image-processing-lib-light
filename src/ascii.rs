use crate::color;
use crate::io;
use image::DynamicImage;
use image::GenericImageView;

pub fn convert_to_ascii(img: &DynamicImage, factor: u32) -> String {
    const RAMP: &[u8; 10] = b" .:-=+*#%@";
    let mut ascii = String::new();

    let new_width = img.width() / factor;
    let new_height = ((img.height() as f32 / factor as f32) * 0.5) as u32;

    println!("new width: {} x new height: {}", new_width, new_height);

    let img = io::resize_image(img, new_width, new_height, true);

    println!("resized: {} x {}", img.width(), img.height());
    let img = color::to_grayscale(&img);
    for y in 0..img.height() {
        for x in 0..img.width() {
            let pixel = img.get_pixel(x, y)[0];
            let pixel_normalized = pixel as f64 / 255.0;
            let index = (RAMP.len() - 1) as f64 * pixel_normalized;
            let index = index.floor() as usize;
            ascii.push(RAMP[index] as char);
        }
        ascii.push('\n');
    }
    return ascii;
}
