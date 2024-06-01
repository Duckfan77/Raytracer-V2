use std::env;

use anyhow::Result;
use image::{Rgb, RgbImage};

fn main() -> Result<()> {
    // Image details
    let image_width = 256;
    let image_height = 256;

    // Render
    let mut buf = RgbImage::new(image_width, image_height);

    for j in 0..image_height {
        for i in 0..image_width {
            let r = i as f64 / (image_width - 1) as f64;
            let g = j as f64 / (image_height - 1) as f64;
            let b = 0.0;

            let ir = (255.999 * r) as u8;
            let ig = (255.999 * g) as u8;
            let ib = (255.999 * b) as u8;

            buf.put_pixel(i, j, Rgb([ir, ig, ib]))
        }
    }

    buf.save_with_format(
        &env::args()
            .nth(1)
            .expect("Must provide a file to save the image to"),
        image::ImageFormat::Png,
    )?;

    Ok(())
}
