use crate::color::Color;

pub struct Lambertian {
    pub(super) albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}
