use std::{env, io::Write};

use anyhow::Result;
use color::{write_color, Color};
use hittable::{
    hittable_list::HittableList,
    sphere::{self, Sphere},
    Hittable,
};
use image::RgbImage;
use ray::Ray;
use vec3::{Point3, Vec3};

mod color;
mod hittable;
mod ray;
mod vec3;

fn ray_color(r: &Ray, world: &Hittable) -> Color {
    if let Some(rec) = world.hit(r, 0.0, f64::INFINITY) {
        return 0.5 * (Color::from(rec.normal.into()) + Color::white());
    }

    // Basic gradient. This is expected to have a small horizontal gradient to go with the vertical gradient,
    // due to normalizing the direction before taking the y coordinate.

    let unit_direction = r.direction().unit_vector();
    let a = 0.5 * (unit_direction.y() + 1.0); // convert y coordinate to between 0 and 1
    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}

fn main() -> Result<()> {
    // Image details
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 480;

    // Calculate image height, ensure it's at least 1
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let image_height = if image_height > 0 { image_height } else { 1 };

    // World
    let mut world = HittableList::new();
    world.add(Hittable::Sphere(Sphere::new(
        &Point3::new(0., 0., -1.),
        0.5,
    )));
    world.add(Hittable::Sphere(Sphere::new(
        &Point3::new(0., -100.5, -1.),
        100.,
    )));
    let world = &Hittable::HittableList(world);

    // Camera
    let focal_length = 1.0; // Distance between the camera center and the viewport
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    // Viewport Vectors
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0); // Horizontal across the viewport, starting at the left
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0); // Vertical across the viewport, starting at the top

    // Viewport pixel delta vectors
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    // Find upper left pixel
    let viewport_upper_left = // upper left corner of the viewport
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel_00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v); // First pixel is half a pixel delta from the top left corner

    // Render
    let mut buf = RgbImage::new(image_width, image_height);

    let mut stdout = std::io::stdout().lock();
    for j in 0..image_height {
        write!(stdout, "\rScanlines remaining: {}", image_height - j)?;
        stdout.flush()?;
        for i in 0..image_width {
            let pixel_center =
                pixel_00_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(&camera_center, &ray_direction);

            let pixel_color = ray_color(&r, world);

            write_color(&mut buf, &pixel_color, i, j)
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
