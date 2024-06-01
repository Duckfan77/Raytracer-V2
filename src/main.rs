use std::{env, io::Write};

use anyhow::Result;
use color::{write_color, Color};
use image::RgbImage;

mod color;
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
            write_color(
                &mut buf,
                &Color::new(
                    i as f64 / (image_width - 1) as f64,
                    j as f64 / (image_height - 1) as f64,
                    0.0,
                ),
                i,
                j,
            )
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
