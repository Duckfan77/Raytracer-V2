use std::sync::Arc;

use anyhow::Result;
use camera::Camera;
use color::Color;
use hittable::{hittable_list::HittableList, sphere::Sphere};

use material::{lambertian::Lambertian, Material};
use vec3::Point3;

mod camera;
mod color;
mod hittable;
mod interval;
mod material;
mod ray;
mod vec3;

fn main() -> Result<()> {
    // World
    let mat: Arc<Material> = Lambertian::new(Color::new(0.5, 0.5, 0.5)).into();
    let mut world_list = HittableList::new();
    world_list.add(Sphere::new(&Point3::new(0., 0., -1.), 0.5, mat.clone()).into());
    world_list.add(Sphere::new(&Point3::new(0., -100.5, -1.), 100., mat.clone()).into());
    let world = world_list.into();

    // Camera and Render
    let cam = Camera {
        aspect_ratio: 16.0 / 9.0,
        image_width: 480,
        samples_per_pixel: 100,
        max_depth: 50,
    };

    cam.render(&world)
}
