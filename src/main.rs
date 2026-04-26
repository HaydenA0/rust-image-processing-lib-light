mod getinfo;
mod io;
mod transformcolor;

fn main() {
    let mut image = io::load_image_raw("images/dog_in_car.jpg").unwrap();
    let gray_image = transformcolor::make_grasyscale(&mut image);
    let histogram = getinfo::get_image_histogram(&gray_image);
    let histogram_normalized = getinfo::get_image_histogram_normalized(&histogram);
    getinfo::print_histogram_normalized(&histogram_normalized);
}
