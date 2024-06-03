use anyhow::Result;

use scene::*;

mod camera;
mod color;
mod hittable;
mod interval;
mod material;
mod ray;
mod scene;
mod vec3;

fn main() -> Result<()> {
    let world = two_spheres();
    let cam = default_camera();

    cam.render(&world)
}
