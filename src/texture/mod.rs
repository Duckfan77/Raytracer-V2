pub mod checker;
pub mod solid_color;

use crate::{color::Color, vec3::Point3};

#[derive(Clone)]
#[non_exhaustive]
pub enum Texture {
    SolidColor(solid_color::SolidColor),
    Checker(checker::Checker),
}

impl From<solid_color::SolidColor> for Texture {
    fn from(value: solid_color::SolidColor) -> Self {
        Texture::SolidColor(value)
    }
}

impl From<checker::Checker> for Texture {
    fn from(value: checker::Checker) -> Self {
        Texture::Checker(value)
    }
}

impl Texture {
    pub fn value(&self, u: f64, v: f64, p: Point3) -> Color {
        use Texture::*;
        match self {
            SolidColor(c) => c.albedo,
            Checker(c) => {
                let x_int = (c.inv_scale * p.x()).floor() as i32;
                let y_int = (c.inv_scale * p.y()).floor() as i32;
                let z_int = (c.inv_scale * p.z()).floor() as i32;

                let is_even = (x_int + y_int + z_int) % 2 == 0;

                if is_even {
                    c.even.value(u, v, p)
                } else {
                    c.odd.value(u, v, p)
                }
            }
        }
    }
}
