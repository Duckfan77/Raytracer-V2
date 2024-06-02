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
    let (world, cam) = two_spheres();

    cam.render(&world)
}
