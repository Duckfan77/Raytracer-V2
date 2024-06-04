use crate::{
    color::Color,
    texture::{solid_color::SolidColor, Texture},
};

#[derive(Clone)]
pub struct Lambertian {
    pub(super) tex: Texture,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self {
            tex: SolidColor::new(albedo).into(),
        }
    }

    pub fn from_texture(tex: impl Into<Texture>) -> Self {
        Self { tex: tex.into() }
    }
}
