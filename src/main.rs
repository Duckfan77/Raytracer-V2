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
    let world = book2_final();
    let cam = book2_final_camera(400, 250, 4);

    cam.render(&world)
}
