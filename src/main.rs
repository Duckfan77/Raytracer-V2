use std::io::{stdout, Write};

use anyhow::Result;

fn main() -> Result<()> {
    // Image details
    let image_width = 256;
    let image_height = 256;

    // Render
    let mut stdout = stdout().lock();
    write!(stdout, "P3\n{image_width} {image_height}\n255\n")?;

    for j in 0..image_height {
        for i in 0..image_width {
            let r = i as f64 / (image_width - 1) as f64;
            let g = j as f64 / (image_height - 1) as f64;
            let b = 0.0;

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            write!(stdout, "{ir} {ig} {ib}\n")?;
        }
    }

    Ok(())
}
