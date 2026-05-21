use crate::includes::RawImage;
use std::collections::HashMap;

pub fn get_image_info(image: &RawImage) -> (f32, f32, f32, f32, f32, f32, f32) {
    let mean = get_image_mean(image);
    let median = get_image_median(image);
    let max = get_image_max(image);
    let min = get_image_min(image);
    let stddev = get_image_stddev(image, mean);
    return (
        mean,
        median,
        max,
        min,
        stddev,
        image.x_size as f32,
        image.y_size as f32,
    );
}

pub fn get_image_info_string(image: &RawImage) -> String {
    let mean = get_image_mean(image);
    let median = get_image_median(image);
    let max = get_image_max(image);
    let min = get_image_min(image);
    let stddev = get_image_stddev(image, mean);

    format!(
        "Image info:\n\tDimensions: {}x{}\n\tChannels: {}\n\tMean: {}\n\tStddev: {}\n\tMax: {}\n\tMin: {}\n\tMedian: {}",
        image.x_size, image.y_size, image.channels, mean, stddev, max, min, median
    )
}

pub fn get_image_mean(image: &RawImage) -> f32 {
    assert!(image.channels == 1);
    let mut sum = 0.0;
    for &x in &image.data {
        sum += x;
    }
    return sum / image.data.len() as f32;
}

pub fn get_image_stddev(image: &RawImage, mean: f32) -> f32 {
    assert!(image.channels == 1);
    let mut sum = 0.0;
    for &x in &image.data {
        sum += (x - mean).powi(2);
    }
    return (sum / image.data.len() as f32).sqrt();
}

pub fn get_image_max(image: &RawImage) -> f32 {
    assert!(image.channels == 1);
    let mut max = image.data[0];
    for &x in &image.data {
        if x > max {
            max = x;
        }
    }
    return max;
}

pub fn get_image_min(image: &RawImage) -> f32 {
    assert!(image.channels == 1);
    let mut min = image.data[0];
    for &x in &image.data {
        if x < min {
            min = x;
        }
    }
    return min;
}

pub fn get_image_median(image: &RawImage) -> f32 {
    assert!(image.channels == 1);
    let mut sorted = image.data.clone();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    return sorted[sorted.len() / 2];
}

pub fn get_image_histogram(image: &RawImage) -> HashMap<u8, u32> {
    assert!(image.channels == 1);
    let mut histogram = HashMap::new();
    for &x in &image.data {
        let bucket = (x * 255.0) as u8;
        *histogram.entry(bucket).or_insert(0) += 1;
    }
    return histogram;
}

pub fn get_image_histogram_normalized(histogram: &HashMap<u8, u32>) -> HashMap<u8, f32> {
    let mut normalized_histogram = HashMap::new();
    let total = histogram.values().sum::<u32>();
    for (&bucket, count) in histogram {
        let normalized_count = *count as f32 / total as f32;
        *normalized_histogram.entry(bucket).or_insert(0.0) += normalized_count;
    }
    return normalized_histogram;
}

pub fn print_histogram(histogram: &HashMap<u8, u32>) {
    let mut buckets: Vec<_> = histogram.iter().collect();
    buckets.sort_by(|a, b| a.1.cmp(b.1));
    for (bucket, count) in buckets {
        println!("luminance {}: frequency {}", bucket, count);
    }
}

pub fn print_histogram_normalized(histogram: &HashMap<u8, f32>) {
    let mut buckets: Vec<_> = histogram.iter().collect();
    buckets.sort_by(|a, b| a.1.partial_cmp(b.1).unwrap());
    for (bucket, count) in buckets {
        println!("luminance {}: frequency {}", bucket, count);
    }
}
