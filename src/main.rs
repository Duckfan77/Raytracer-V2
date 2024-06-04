use anyhow::Result;

use scene::*;

mod camera;
mod color;
mod hittable;
mod interval;
mod material;
mod ray;
mod scene;
mod texture;
mod vec3;

fn main() -> Result<()> {
    let world = earth();
    let cam = earth_camera();

    cam.render(&world)
}
