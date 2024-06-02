use std::{env, f64::INFINITY, io::Write};

use anyhow::Result;
use image::RgbImage;
use rand::distributions::{Distribution, Uniform};

use crate::{
    color::{write_color, Color},
    hittable::Hittable,
    ray::Ray,
    vec3::{Point3, Vec3},
};

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u32,
    pub samples_per_pixel: u32,
}

impl Camera {
    #[allow(dead_code)]
    pub fn new_basic() -> Self {
        Self {
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 10,
        }
    }

    pub fn render(&self, world: &Hittable) -> Result<()> {
        CameraCore::initialize(self).render(world)
    }
}

struct CameraCore {
    #[allow(dead_code)]
    aspect_ratio: f64, // Ratio of image width over height
    image_width: u32,       // Rendered image width in pixel count
    samples_per_pixel: u32, // Number of samples for each pixel

    image_height: u32,        // Rendered image height
    pixel_samples_scale: f64, // Color scale factor for a sum of pixel samples
    center: Point3,           // Camera center
    pixel_00_loc: Point3,     // Location of pixel 0, 0
    pixel_delta_u: Point3,    // Offset to pixel to the right
    pixel_delta_v: Point3,    // Offset to pixel below
}

impl CameraCore {
    fn render(&self, world: &Hittable) -> Result<()> {
        let mut buf = RgbImage::new(self.image_width, self.image_height);

        let mut stdout = std::io::stdout().lock();
        for j in 0..self.image_height {
            write!(
                stdout,
                "\rScanlines remaining: {}                         ",
                self.image_height - j
            )?;
            stdout.flush()?;
            for i in 0..self.image_width {
                let mut pixel_color = Color::black();
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += self.ray_color(&r, world)
                }

                write_color(&mut buf, &(pixel_color * self.pixel_samples_scale), i, j)
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

    fn initialize(params: &Camera) -> Self {
        let aspect_ratio = params.aspect_ratio;
        let image_width = params.image_width;
        let samples_per_pixel = params.samples_per_pixel;

        let image_height = (image_width as f64 / aspect_ratio) as u32;
        let image_height = if image_height < 1 { 1 } else { image_height };

        let pixel_samples_scale = 1.0 / samples_per_pixel as f64;

        let center = Point3::new(0., 0., 0.);

        // Viewport Dimensions
        let focal_length = 1.0; // Distance between the camera center and the viewport
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        // Viewport Vectors
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0); // Horizontal across the viewport, starting at the left
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0); // Vertical across the viewport, starting at the top

        // Viewport pixel delta vectors
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // Find upper left pixel
        let viewport_upper_left = // upper left corner of the viewport
        center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel_00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v); // First pixel is half a pixel delta from the top left corner

        Self {
            aspect_ratio,
            image_width,
            samples_per_pixel,
            image_height,
            pixel_samples_scale,
            center,
            pixel_00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    fn ray_color(&self, r: &Ray, world: &Hittable) -> Color {
        if let Some(rec) = world.hit(r, 0.0..=INFINITY) {
            return 0.5 * (Color::from(rec.normal) + Color::white());
        }

        // Basic gradient. This is expected to have a small horizontal gradient to go with the vertical gradient,
        // due to normalizing the direction before taking the y coordinate.

        let unit_direction = r.direction().unit_vector();
        let a = 0.5 * (unit_direction.y() + 1.0); // convert y coordinate to between 0 and 1
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }

    ///
    /// Constructs a camera ray originating from the origin and directed at a randomly
    /// sampled point around the pixel at (i, j)
    ///
    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = sample_square();
        let pixel_sample = self.pixel_00_loc
            + ((i as f64 + offset.x()) * self.pixel_delta_u)
            + ((j as f64 + offset.y()) * self.pixel_delta_v);

        let ray_dir = pixel_sample - self.center;

        Ray::new(&self.center, &ray_dir)
    }
}

static SQUARE_DIST: once_cell::sync::Lazy<Uniform<f64>> =
    once_cell::sync::Lazy::new(|| Uniform::from(-0.5..0.5));

fn sample_square() -> Vec3 {
    let mut rng = rand::thread_rng();
    Vec3::new(
        SQUARE_DIST.sample(&mut rng),
        SQUARE_DIST.sample(&mut rng),
        0.0,
    )
}
