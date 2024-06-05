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
    let world = quads();
    let cam = quads_camera();

    cam.render(&world)
}
