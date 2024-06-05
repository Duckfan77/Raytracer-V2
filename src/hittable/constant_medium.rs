use crate::{
    color::Color,
    material::{isotropic::Isotropic, Material},
    texture::Texture,
};

use super::Hittable;

#[derive(Clone)]
pub struct ConstantMedium {
    pub(super) boundary: Box<Hittable>,
    pub(super) neg_inv_density: f64,
    pub(super) phase_function: Material,
}

impl ConstantMedium {
    pub fn new(boundary: impl Into<Hittable>, density: f64, albedo: Color) -> Self {
        Self {
            boundary: Box::new(boundary.into()),
            neg_inv_density: -1.0 / density,
            phase_function: Isotropic::new(albedo).into(),
        }
    }

    #[allow(dead_code)]
    pub fn with_texture(
        boundary: impl Into<Hittable>,
        density: f64,
        tex: impl Into<Texture>,
    ) -> Self {
        Self {
            boundary: Box::new(boundary.into()),
            neg_inv_density: -1.0 / density,
            phase_function: Isotropic::from_texture(tex).into(),
        }
    }
}
