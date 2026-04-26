mod ascii;
mod color;
mod io;

fn main() {
    let path = "images/dog_in_car.jpg";
    let img = match io::read_image(path) {
        Ok(img) => img,
        Err(e) => panic!("Error: {}", e),
    };

    let ascii = ascii::convert_to_ascii(&img, 3);
    println!("{}", ascii);

    println!("Hello, world!");
}
