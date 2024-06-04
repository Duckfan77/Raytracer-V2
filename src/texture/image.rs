use std::sync::Arc;

use image::io::Reader as ImageReader;
use image::RgbImage;

#[derive(Clone)]
pub struct Image {
    pub(super) image: Arc<RgbImage>,
}

impl Image {
    pub fn new(filename: &str) -> Self {
        let image = ImageReader::open(filename)
            .expect("Cannot read image file")
            .decode()
            .expect("Failed to decode image")
            .to_rgb8();
        Self {
            image: Arc::new(image),
        }
    }
}
