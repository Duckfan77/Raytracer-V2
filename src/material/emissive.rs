use crate::{
    color::Color,
    texture::{solid_color::SolidColor, Texture},
};

#[derive(Clone)]
pub struct DiffuseLight {
    pub(super) tex: Texture,
}

impl DiffuseLight {
    pub fn new(emit: Color) -> Self {
        Self {
            tex: SolidColor::new(emit).into(),
        }
    }

    pub fn from_texture(tex: impl Into<Texture>) -> Self {
        Self { tex: tex.into() }
    }
}
