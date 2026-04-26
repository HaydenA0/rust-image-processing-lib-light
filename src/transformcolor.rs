use crate::io::RawImage;

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
