use anyhow::Result;
use camera::Camera;
use color::Color;
use hittable::{hittable_list::HittableList, sphere::Sphere};

use material::{
    dielectric::{Dielectric, RI_AIR, RI_GLASS, RI_WATER},
    lambertian::Lambertian,
    metal::Metal,
    Mat,
};
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
    let material_ground: Mat = Lambertian::new(Color::new(0.8, 0.8, 0.0)).into();
    let material_center: Mat = Lambertian::new(Color::new(0.1, 0.2, 0.5)).into();
    let material_left: Mat = Dielectric::new(RI_GLASS).into();
    let material_bubble: Mat = Dielectric::new(RI_AIR / RI_GLASS).into(); // Air within Glass
    let material_right: Mat = Metal::new(Color::new(0.8, 0.6, 0.2), 1.0).into();

    let mut world_list = HittableList::new();
    world_list.add(Sphere::new(
        &Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    ));
    world_list.add(Sphere::new(
        &Point3::new(0.0, 0.0, -1.2),
        0.5,
        material_center,
    ));
    world_list.add(Sphere::new(
        &Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    ));
    world_list.add(Sphere::new(
        &Point3::new(-1.0, 0.0, -1.0),
        0.4,
        material_bubble,
    ));
    world_list.add(Sphere::new(
        &Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    ));

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
