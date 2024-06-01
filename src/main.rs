use std::{env, io::Write};

use anyhow::Result;
use image::{Rgb, RgbImage};

mod vec3;

fn main() -> Result<()> {
    // Image details
    let image_width = 256;
    let image_height = 256;

    // Render
    let mut buf = RgbImage::new(image_width, image_height);

    let mut stdout = std::io::stdout().lock();
    for j in 0..image_height {
        write!(stdout, "\rScanlines remaining: {}", image_height - j)?;
        stdout.flush()?;
        for i in 0..image_width {
            let r = i as f64 / (image_width - 1) as f64;
            let g = j as f64 / (image_height - 1) as f64;
            let b = 0.0;

            let ir = (255.999 * r) as u8;
            let ig = (255.999 * g) as u8;
            let ib = (255.999 * b) as u8;

            buf.put_pixel(i, j, Rgb([ir, ig, ib]));
        }
    }

    buf.save_with_format(
        &env::args().nth(1).unwrap_or("output/test.png".to_string()),
        image::ImageFormat::Png,
    )?;
    write!(
        stdout,
        "\rDone.                                                 \n"
    )?;

    Ok(())
}
