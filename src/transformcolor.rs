use crate::io::RawImage;
// TODO : handle cases where cooardinates are out of bounds

struct Pixel {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

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
        width: image.width,
        height: image.height,
        channels: 1,
    };
}

pub fn rotate_image(image: &RawImage, angle: f64) -> Result<RawImage, String> {
    match angle {
        90.0 => return Ok(rotate90(image)),
        0.0 => return Ok(clone_image(image)),
        180.0 => return Ok(rotate180(image)),
        270.0 => return Ok(rotate270(image)),
        _ => return Err(format!("Invalid angle: {}", angle)),
    }
}

fn clone_image(image: &RawImage) -> RawImage {
    return RawImage {
        data: image.data.clone(),
        width: image.width,
        height: image.height,
        channels: image.channels,
    };
}

pub fn access_pixel_at_coord(image: &RawImage, x: u32, y: u32) -> Pixel {
    if image.channels == 1 {
        let index = y * image.width + x;
        return Pixel {
            r: image.data[index as usize],
            g: image.data[index as usize],
            b: image.data[index as usize],
            a: 1.0,
        };
    } else if image.channels == 3 {
        let r_index = y * image.width * 3 + x * 3;
        let g_index = r_index + 1;
        let b_index = r_index + 2;
        return Pixel {
            r: image.data[r_index as usize],
            g: image.data[g_index as usize],
            b: image.data[b_index as usize],
            a: 1.0,
        };
    } else {
        // TODO : handle other number of channels better
        return Pixel {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.0,
        };
    }
}

pub fn rotate270(image: &RawImage) -> RawImage {
    let mut new_data = Vec::with_capacity(image.data.len());
    let new_width = image.height;
    let new_height = image.width;

    for y in 0..new_height {
        for x in 0..new_width {
            let src_x = image.width - y - 1;
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
        width: image.height,
        height: image.width,
        channels: image.channels,
    }
}

pub fn rotate180(image: &RawImage) -> RawImage {
    let mut new_data = Vec::with_capacity(image.data.len());
    let new_height = image.height;
    let new_width = image.width;

    for y in 0..new_height {
        for x in 0..new_width {
            let src_x = image.width - x - 1;
            let src_y = image.height - y - 1;

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
        width: image.width,
        height: image.height,
        channels: image.channels,
    };
}

pub fn rotate90(image: &RawImage) -> RawImage {
    let mut new_data = Vec::with_capacity(image.data.len());
    let new_height = image.width;
    let new_width = image.height;

    for y in 0..new_height {
        for x in 0..new_width {
            let src_x = y;
            let src_y = image.height - x - 1;
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
        width: image.height,
        height: image.width,
        channels: image.channels,
    };
}

pub fn change_pixel_at_coord(image: &mut RawImage, x: u32, y: u32, pixel: Pixel) {
    if image.channels == 1 {
        let index = y * image.width + x;
        image.data[index as usize] = pixel.r;
    } else if image.channels == 3 {
        let r_index = y * image.width * 3 + x * 3;
        let g_index = r_index + 1;
        let b_index = r_index + 2;
        image.data[r_index as usize] = pixel.r;
        image.data[g_index as usize] = pixel.g;
        image.data[b_index as usize] = pixel.b;
    }
}
