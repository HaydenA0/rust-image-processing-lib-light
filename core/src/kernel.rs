use crate::includes::RawImage;

pub fn normalize_kernel(kernel: &[Vec<f32>]) -> Vec<Vec<f32>> {
    let mut sum = 0.0;
    for row in kernel {
        for val in row {
            sum += val;
        }
    }
    return kernel
        .iter()
        .map(|row| row.iter().map(|val| val / sum).collect())
        .collect();
}

pub fn apply_kernel(image: &RawImage, kernel: &[Vec<f32>]) -> RawImage {
    let kernel_size = kernel.len() as i32;
    let half = kernel_size / 2;
    let width = image.x_size as i32;
    let height = image.y_size as i32;

    let mut new_data = vec![0.0; (image.x_size * image.y_size) as usize];

    for y in 0..height {
        for x in 0..width {
            let mut acc = 0.0;

            for ky in 0..kernel_size {
                for kx in 0..kernel_size {
                    let ix = x + kx - half;
                    let iy = y + ky - half;

                    if ix >= 0 && ix < width && iy >= 0 && iy < height {
                        let idx = (iy * width + ix) as usize;
                        acc += image.data[idx] * kernel[ky as usize][kx as usize];
                    }
                }
            }

            let out_idx = (y * width + x) as usize;
            acc = acc.clamp(0.0, 1.0);
            new_data[out_idx] = acc;
        }
    }

    RawImage {
        data: new_data,
        x_size: image.x_size,
        y_size: image.y_size,
        channels: 1,
    }
}

pub fn make_gaussian_kernel(size: usize, sigma: f32) -> Vec<Vec<f32>> {
    let mut kernel = vec![vec![0.0; size]; size];
    let half = (size / 2) as usize;
    let sigma2 = sigma * sigma;
    let mut sum = 0.0;

    for y in 0..size {
        for x in 0..size {
            let new_x = x as f32 - half as f32;
            let new_y = y as f32 - half as f32;
            kernel[y][x] = 1.0 / (2.0 * std::f32::consts::PI * sigma2)
                * (-(new_x * new_x + new_y * new_y) / sigma2).exp();
            sum += kernel[y][x];
        }
    }
    // normalize kernel
    for y in 0..size {
        for x in 0..size {
            kernel[y][x] /= sum;
        }
    }

    return kernel;
}

pub fn apply_gaussian_blur(image: &RawImage, sigma: f32, size: usize) -> RawImage {
    let kernel = make_gaussian_kernel(size, sigma);
    apply_kernel(image, &kernel)
}
