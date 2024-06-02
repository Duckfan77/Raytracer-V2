pub mod lambertian;

use std::sync::Arc;

use lambertian::Lambertian;

use crate::{color::Color, hittable::HitRecord, ray::Ray, vec3::Vec3};

#[non_exhaustive]
pub enum Material {
    Lambertian(lambertian::Lambertian),
}

impl From<lambertian::Lambertian> for Arc<Material> {
    fn from(value: lambertian::Lambertian) -> Self {
        Arc::new(Material::Lambertian(value))
    }
}

impl From<lambertian::Lambertian> for Material {
    fn from(value: lambertian::Lambertian) -> Self {
        Material::Lambertian(value)
    }
}

impl Material {
    ///
    /// Return:
    /// - `None`, when no scattering occurs
    /// - `Some(attenuation, bounced_ray)` when scattering occurs. `attenuation`
    /// defines how much and in what color the ray should be attenuated by this
    /// bounce
    ///
    pub fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        use Material::*;
        match self {
            Lambertian(l) => {
                let mut scatter_dir = rec.normal + Vec3::random_unit_vector();

                // Catch degenerate scatter direction
                if scatter_dir.near_zero() {
                    scatter_dir = rec.normal
                }

                let scattered = Ray::new(&rec.p, &scatter_dir);
                Some((l.albedo, scattered))
            }
        }
    }
}
