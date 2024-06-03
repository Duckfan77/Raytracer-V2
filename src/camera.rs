use std::{env, f64::INFINITY, io::Write};

use anyhow::Result;
use image::RgbImage;
use rand::distributions::{Distribution, Uniform};
use rayon::prelude::*;

use crate::{
    color::{write_row, Color},
    hittable::Hittable,
    ray::Ray,
    vec3::{Point3, Vec3},
};

pub struct Camera {
    pub aspect_ratio: f64,      // Ratio of image width over height
    pub image_width: u32,       // Rendered image width in pixel count
    pub samples_per_pixel: u32, // Number of samples for each pixel
    pub max_depth: u32,         // Maximum number of ray bounces into scene
    pub vfov: f64,              // Vertical view angle (field of view) in degrees
    pub look_from: Point3,      // Point camera is looking from
    pub look_at: Point3,        // Point camera is looking at
    pub v_up: Vec3,             // Camera-relative "up" direction
    pub defocus_angle: f64,     // Variation angle of rays through each pixel
    pub focus_dist: f64,        // Distance from camera look_from point to plane of perfect focus
}

impl Camera {
    #[allow(dead_code)]
    pub fn new_basic() -> Self {
        Self {
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 10,
            max_depth: 10,
            vfov: 90.0,
            look_from: Point3::new(0.0, 0.0, 0.0),
            look_at: Point3::new(0.0, 0.0, -1.0),
            v_up: Vec3::new(0.0, 1.0, 0.0),
            defocus_angle: 0.0,
            focus_dist: 10.0,
        }
    }

    pub fn render(&self, world: &Hittable) -> Result<()> {
        CameraCore::initialize(self).render(world)
    }
}

struct CameraCore {
    image_width: u32,       // Rendered image width in pixel count
    samples_per_pixel: u32, // Number of samples for each pixel
    max_depth: u32,         // Maximum number of ray bounces into scene

    image_height: u32,        // Rendered image height
    pixel_samples_scale: f64, // Color scale factor for a sum of pixel samples
    center: Point3,           // Camera center
    pixel_00_loc: Point3,     // Location of pixel 0, 0
    pixel_delta_u: Point3,    // Offset to pixel to the right
    pixel_delta_v: Point3,    // Offset to pixel below

    #[allow(dead_code)]
    u: Vec3, // Camera Frame Basis Vector: Left relative to camera
    #[allow(dead_code)]
    v: Vec3, // Camera Frame Basis Vector: Up relative to camera
    #[allow(dead_code)]
    w: Vec3, // Camera Frame Basis Vector: Behind relative to camera (we look along the -w axis)

    defocus_angle: f64,   // Variation angle of rays through each pixel
    defocus_disk_u: Vec3, // Defocus disk horizontal radius
    defocus_disk_v: Vec3, // Defocus disk vertical radius
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
            let row: Vec<Color> = (0..self.image_width)
                .into_par_iter()
                .map(|i| {
                    (0..self.samples_per_pixel)
                        .into_iter()
                        .map(|_| {
                            let r = self.get_ray(i, j);
                            self.ray_color(r, self.max_depth, world)
                        })
                        .sum::<Color>()
                        * self.pixel_samples_scale
                })
                .collect();

            write_row(&mut buf, &row, j)
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
        let image_width = params.image_width;
        let samples_per_pixel = params.samples_per_pixel;
        let max_depth = params.max_depth;
        let vfov = params.vfov;
        let defocus_angle = params.defocus_angle;

        let image_height = (image_width as f64 / params.aspect_ratio) as u32;
        let image_height = if image_height < 1 { 1 } else { image_height };

        let pixel_samples_scale = 1.0 / samples_per_pixel as f64;

        let center = params.look_from;

        // Viewport Dimensions
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * params.focus_dist;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        // Calculate the u, v, w unit basis vectors for the camera coordinate frame.
        let w = (params.look_from - params.look_at).unit_vector();
        let u = (params.v_up.cross(w)).unit_vector();
        let v = w.cross(u);

        // Viewport Vectors
        let viewport_u = viewport_width * u; // Horizontal across the viewport, starting at the left
        let viewport_v = viewport_height * -v; // Vertical across the viewport, starting at the top

        // Viewport pixel delta vectors
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // Find upper left pixel
        let viewport_upper_left = // upper left corner of the viewport
        center - (params.focus_dist * w) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel_00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v); // First pixel is half a pixel delta from the top left corner

        // Calculate the camera defocus disk basis vectors
        let defocus_radius = params.focus_dist * (defocus_angle / 2.0).to_radians().tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Self {
            image_width,
            samples_per_pixel,
            max_depth,

            image_height,
            pixel_samples_scale,
            center,
            pixel_00_loc,
            pixel_delta_u,
            pixel_delta_v,

            u,
            v,
            w,

            defocus_angle,
            defocus_disk_u,
            defocus_disk_v,
        }
    }

    fn ray_color(&self, r: Ray, depth: u32, world: &Hittable) -> Color {
        // Used to solve shadow acne problem, preventing rays from colliding with the same surface they just did
        const SURFACE_HOLDOFF_DIST: f64 = 0.001;

        if depth <= 0 {
            // exceeded bounce limit, no more light gathered
            return Color::black();
        }

        if let Some(rec) = world.hit(r, SURFACE_HOLDOFF_DIST..=INFINITY) {
            if let Some((attenuation, scattered)) = rec.mat.scatter(r, &rec) {
                return attenuation * self.ray_color(scattered, depth - 1, world);
            }
            return Color::black();
        }

        // Basic gradient. This is expected to have a small horizontal gradient to go with the vertical gradient,
        // due to normalizing the direction before taking the y coordinate.

        let unit_direction = r.direction().unit_vector();
        let a = 0.5 * (unit_direction.y() + 1.0); // convert y coordinate to between 0 and 1
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }

    ///
    /// Constructs a camera ray originating from the defocus disk and directed at a randomly
    /// sampled point around the pixel at (i, j)
    ///
    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = sample_square();
        let pixel_sample = self.pixel_00_loc
            + ((i as f64 + offset.x()) * self.pixel_delta_u)
            + ((j as f64 + offset.y()) * self.pixel_delta_v);

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };

        let ray_dir = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_dir)
    }

    ///
    /// Returns a random point in the camera defocus disk.
    ///
    fn defocus_disk_sample(&self) -> Point3 {
        let p = Vec3::random_in_unit_disk();
        self.center + (p.x() * self.defocus_disk_u + p.y() * self.defocus_disk_v)
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
