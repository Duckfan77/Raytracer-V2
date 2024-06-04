use crate::{color::Color, texture::solid_color::SolidColor};

use super::Texture;

#[derive(Clone)]
pub struct Checker {
    pub(super) even: Box<Texture>,
    pub(super) odd: Box<Texture>,
    pub(super) inv_scale: f64,
}

impl Checker {
    pub fn from_textures(scale: f64, even: impl Into<Texture>, odd: impl Into<Texture>) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even: Box::new(even.into()),
            odd: Box::new(odd.into()),
        }
    }

    pub fn from_colors(scale: f64, c1: Color, c2: Color) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even: Box::new(SolidColor::new(c1).into()),
            odd: Box::new(SolidColor::new(c2).into()),
        }
    }
}
