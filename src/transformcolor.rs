use crate::includes::{Pixel, RawImage, access_pixel_at_coord};

// TODO : handle cases where cooardinates are out of bounds

pub fn make_grasyscale(image: &mut RawImage) -> RawImage {
    assert!(image.channels == 3);
    let mut new_data = Vec::new();
    for x in image.data.chunks(3) {
        let r = x[0];
        let g = x[1];
        let b = x[2];
        new_data.push((r + g + b) / 3.0);
    }
    return RawImage {
        data: new_data,
        x_size: image.x_size,
        y_size: image.y_size,
        channels: 1,
    };
}

pub fn rotate_image(image: &RawImage, angle: u32) -> Result<RawImage, String> {
    match angle {
        90 => return Ok(rotate90(image)),
        0 => return Ok(clone_image(image)),
        180 => return Ok(rotate180(image)),
        270 => return Ok(rotate270(image)),
        _ => return Err(format!("Invalid angle: {}", angle)),
    }
}

fn clone_image(image: &RawImage) -> RawImage {
    return RawImage {
        data: image.data.clone(),
        x_size: image.x_size,
        y_size: image.y_size,
        channels: image.channels,
    };
}

pub fn rotate270(image: &RawImage) -> RawImage {
    let mut new_data = Vec::with_capacity(image.data.len());
    let new_width = image.y_size;
    let new_height = image.x_size;

    for y in 0..new_height {
        for x in 0..new_width {
            let src_x = image.x_size - y - 1;
            let src_y = x;

            let pixel = access_pixel_at_coord(image, src_x, src_y);
            if image.channels == 1 {
                new_data.push(pixel.r);
            }
            if image.channels == 3 {
                new_data.push(pixel.r);
                new_data.push(pixel.g);
                new_data.push(pixel.b);
            }
        }
    }
    assert!(new_data.len() == image.data.len());
    RawImage {
        data: new_data,
        x_size: image.y_size,
        y_size: image.x_size,
        channels: image.channels,
    }
}

pub fn rotate180(image: &RawImage) -> RawImage {
    let mut new_data = Vec::with_capacity(image.data.len());
    let new_height = image.y_size;
    let new_width = image.x_size;

    for y in 0..new_height {
        for x in 0..new_width {
            let src_x = image.x_size - x - 1;
            let src_y = image.y_size - y - 1;

            let pixel = access_pixel_at_coord(image, src_x, src_y);
            if image.channels == 1 {
                new_data.push(pixel.r);
            }
            if image.channels == 3 {
                new_data.push(pixel.r);
                new_data.push(pixel.g);
                new_data.push(pixel.b);
            }
        }
    }
    assert!(new_data.len() == image.data.len());
    return RawImage {
        data: new_data,
        x_size: image.x_size,
        y_size: image.y_size,
        channels: image.channels,
    };
}

pub fn rotate90(image: &RawImage) -> RawImage {
    let mut new_data = Vec::with_capacity(image.data.len());
    let new_height = image.x_size;
    let new_width = image.y_size;

    for y in 0..new_height {
        for x in 0..new_width {
            let src_x = y;
            let src_y = image.y_size - x - 1;
            let pixel = access_pixel_at_coord(image, src_x, src_y);
            if image.channels == 1 {
                new_data.push(pixel.r);
            }
            if image.channels == 3 {
                new_data.push(pixel.r);
                new_data.push(pixel.g);
                new_data.push(pixel.b);
            }
        }
    }
    return RawImage {
        data: new_data,
        x_size: image.y_size,
        y_size: image.x_size,
        channels: image.channels,
    };
}

pub fn change_pixel_at_coord(image: &mut RawImage, x: u32, y: u32, pixel: Pixel) {
    if image.channels == 1 {
        let index = y * image.x_size + x;
        image.data[index as usize] = pixel.r;
    } else if image.channels == 3 {
        let r_index = y * image.x_size * 3 + x * 3;
        let g_index = r_index + 1;
        let b_index = r_index + 2;
        image.data[r_index as usize] = pixel.r;
        image.data[g_index as usize] = pixel.g;
        image.data[b_index as usize] = pixel.b;
    }
}
