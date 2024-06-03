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
    let world = bouncing_random_spheres();
    let cam = random_spheres_camera_fast();

    cam.render(&world)
}
