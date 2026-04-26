use image::DynamicImage;
use image::imageops::FilterType;
use image::io::Reader as ImageReader;

pub fn read_image(path: &str) -> Result<DynamicImage, String> {
    let img = ImageReader::open(path)
        .map_err(|e| format!("Error opening image: {}", e))?
        .decode()
        .map_err(|e| format!("Error decoding image: {}", e))?;
    Ok(img)
}

pub fn save_image(path: &str, img: &DynamicImage) -> Result<(), String> {
    img.save(path)
        .map_err(|e| format!("Error saving image: {}", e))
}

pub fn resize_image(img: &DynamicImage, width: u32, height: u32, exact: bool) -> DynamicImage {
    if exact {
        img.resize_exact(width, height, FilterType::Nearest)
    } else {
        img.resize(width, height, FilterType::Nearest)
    }
}

pub fn crop_image(img: &DynamicImage, x: u32, y: u32, width: u32, height: u32) -> DynamicImage {
    img.crop_imm(x, y, width, height)
}

pub fn rotate_image(img: &DynamicImage, angle: f64) -> Result<DynamicImage, String> {
    match angle {
        90.0 => Ok(img.rotate90()),
        180.0 => Ok(img.rotate180()),
        270.0 => Ok(img.rotate270()),
        _ => Err(format!(
            "Invalid angle: {}, valid values are 90, 180, and 270",
            angle
        )),
    }
}

pub fn flip_image(img: &DynamicImage, horizontal: bool, vertical: bool) -> DynamicImage {
    let mut flipped = img.clone();
    if horizontal {
        flipped = flipped.fliph();
    }
    if vertical {
        flipped = flipped.flipv();
    }
    flipped
}
