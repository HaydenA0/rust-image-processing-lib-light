mod getinfo;
mod io;
mod transformcolor;

fn main() {
    let mut image = io::load_image_raw("images/dog_in_car.jpg").unwrap();
    let gray_image = transformcolor::make_grasyscale(&mut image);
    let rotated_image = transformcolor::rotate_image(&image, 270).unwrap();
    io::save_image_raw("images/dog_in_car_rotated.jpg", &rotated_image);
}
