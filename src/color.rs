use image::DynamicImage;

pub fn to_grayscale(img: &DynamicImage) -> DynamicImage {
    img.grayscale()
}
