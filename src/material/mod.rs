pub mod lambertian;
pub mod metal;

use std::sync::Arc;

use crate::{color::Color, hittable::HitRecord, ray::Ray, vec3::Vec3};

#[non_exhaustive]
pub enum Material {
    Lambertian(lambertian::Lambertian),
    Metal(metal::Metal),
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

impl From<metal::Metal> for Arc<Material> {
    fn from(value: metal::Metal) -> Self {
        Arc::new(Material::Metal(value))
    }
}

impl From<metal::Metal> for Material {
    fn from(value: metal::Metal) -> Self {
        Material::Metal(value)
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

            Metal(m) => {
                let reflected = r_in.direction().reflect(&rec.normal);
                let scattered = Ray::new(&rec.p, &reflected);
                Some((m.albedo, scattered))
            }
        }
    }
}
