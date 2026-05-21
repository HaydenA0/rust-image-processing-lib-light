mod io;

use image_core::kernel;
use image_core::transformcolor;

fn main() {
    let input_image_path = "../images/dog_in_car.jpg";
    let output_image_path = "../images/dog_in_car_sobel.jpg";
    let mut image = io::load_image_raw(input_image_path).unwrap();

    let gray_image = transformcolor::make_grasyscale(&mut image);
    let sobel_image = kernel::apply_sobel(&gray_image);
    _ = io::save_image_raw(output_image_path, &sobel_image);
    println!(
        "Full path: {}",
        std::path::Path::new(input_image_path)
            .canonicalize()
            .unwrap()
            .display()
    );
}
