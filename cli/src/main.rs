mod io;

use image_core::kernel;
use image_core::transformcolor;

fn main() {
    let input_image_path = "./images/dog_in_car.jpg";
    let output_image_path = "./images/dog_in_car_sobel_looped.jpg";
    let mut image = io::load_image_raw(input_image_path).unwrap();
    let loop_size = 10;

    let gray_image = transformcolor::make_grasyscale(&mut image);
    let mut sobel_image = kernel::apply_sobel(&gray_image);

    for _ in 0..loop_size {
        sobel_image = kernel::apply_sobel(&sobel_image);
        sobel_image = kernel::apply_gaussian_blur(&sobel_image, 1.0, 3);
    }
    _ = io::save_image_raw(output_image_path, &sobel_image);
    println!(
        "Full path: {}",
        std::path::Path::new(input_image_path)
            .canonicalize()
            .unwrap()
            .display()
    );
}
