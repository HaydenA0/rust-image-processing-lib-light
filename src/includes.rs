use std::ops::{Add, Sub};
pub struct RawImage {
    pub data: Vec<f32>,
    pub x_size: u32,
    pub y_size: u32,
    pub channels: usize,
}

impl Clone for RawImage {
    fn clone(&self) -> Self {
        return RawImage {
            data: self.data.clone(),
            x_size: self.x_size,
            y_size: self.y_size,
            channels: self.channels,
        };
    }
}

impl Add for RawImage {
    type Output = RawImage;
    fn add(self, rhs: RawImage) -> Self::Output {
        assert_eq!(self.x_size, rhs.x_size);
        assert_eq!(self.y_size, rhs.y_size);
        assert_eq!(self.channels, rhs.channels);
        let mut new_data = Vec::new();
        for i in 0..self.data.len() {
            new_data.push(self.data[i] + rhs.data[i]);
        }
        return RawImage {
            data: new_data,
            x_size: self.x_size,
            y_size: self.y_size,
            channels: self.channels,
        };
    }
}

impl Sub for RawImage {
    type Output = RawImage;
    fn sub(self, rhs: RawImage) -> Self::Output {
        assert_eq!(self.x_size, rhs.x_size);
        assert_eq!(self.y_size, rhs.y_size);
        assert_eq!(self.channels, rhs.channels);
        let mut new_data = Vec::new();
        for i in 0..self.data.len() {
            new_data.push(self.data[i] - rhs.data[i]);
        }
        return RawImage {
            data: new_data,
            x_size: self.x_size,
            y_size: self.y_size,
            channels: self.channels,
        };
    }
}

pub struct Pixel {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

pub fn access_pixel_at_coord(image: &RawImage, x: u32, y: u32) -> Pixel {
    if image.channels == 1 {
        let index = y * image.x_size + x;
        return Pixel {
            r: image.data[index as usize],
            g: image.data[index as usize],
            b: image.data[index as usize],
            a: 1.0,
        };
    } else if image.channels == 3 {
        let r_index = y * image.x_size * 3 + x * 3;
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
