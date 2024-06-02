use anyhow::Result;
use camera::Camera;
use hittable::{hittable_list::HittableList, sphere::Sphere, Hittable};

use vec3::Point3;

mod camera;
mod color;
mod hittable;
mod interval;
mod ray;
mod vec3;

fn main() -> Result<()> {
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

    // Camera and Render
    let cam = Camera {
        aspect_ratio: 16.0 / 9.0,
        image_width: 480,
        samples_per_pixel: 100,
        max_depth: 50,
    };

    cam.render(world)
}
