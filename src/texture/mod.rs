pub mod solid_color;

use crate::{color::Color, vec3::Point3};

#[derive(Clone)]
#[non_exhaustive]
pub enum Texture {
    SolidColor(solid_color::SolidColor),
}

impl From<solid_color::SolidColor> for Texture {
    fn from(value: solid_color::SolidColor) -> Self {
        Texture::SolidColor(value)
    }
}

impl Texture {
    pub fn value(&self, _u: f64, _v: f64, _p: Point3) -> Color {
        use Texture::*;
        match self {
            SolidColor(c) => c.albedo,
        }
    }
}
