mod getinfo;
mod includes;
mod io;
mod kernel;
mod operation;
mod transformcolor;

fn main() {
    let mut image = io::load_image_raw("images/dog_in_car.jpg").unwrap();
    let gray_image = transformcolor::make_grasyscale(&mut image);
    let kernel = kernel::make_gaussian_kernel(7, 9.0);
    let new_image = kernel::apply_kernel(&gray_image, &kernel);
    let diff = new_image.clone() - gray_image;
    io::save_image_raw("images/dog_in_car_kernel.jpg", &new_image).unwrap();
    io::save_image_raw("images/dog_in_car_diff.jpg", &diff).unwrap();
}
