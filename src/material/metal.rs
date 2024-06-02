use crate::color::Color;

pub struct Metal {
    pub(super) albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}
