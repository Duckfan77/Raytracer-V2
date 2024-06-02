use crate::{color::Color, hittable::HitRecord, ray::Ray};

#[non_exhaustive]
pub enum Material {}

impl Material {
    ///
    /// Return:
    /// - `None`, when no scattering occurs
    /// - `Some(attenuation, bounced_ray)` when scattering occurs. `attenuation`
    /// defines how much and in what color the ray should be attenuated by this
    /// bounce
    ///
    pub fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        match self {
            _ => None,
        }
    }
}
