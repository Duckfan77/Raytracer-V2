pub mod dielectric;
pub mod lambertian;
pub mod metal;

use std::sync::Arc;

use dielectric::refract;

use crate::{color::Color, hittable::HitRecord, ray::Ray, vec3::Vec3};

pub type Mat = Arc<Material>;

#[non_exhaustive]
pub enum Material {
    Lambertian(lambertian::Lambertian),
    Metal(metal::Metal),
    Dielectric(dielectric::Dielectric),
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

impl From<dielectric::Dielectric> for Arc<Material> {
    fn from(value: dielectric::Dielectric) -> Self {
        Arc::new(Material::Dielectric(value))
    }
}

impl From<dielectric::Dielectric> for Material {
    fn from(value: dielectric::Dielectric) -> Self {
        Material::Dielectric(value)
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
                let mut reflected = r_in.direction().reflect(&rec.normal);
                reflected = reflected.unit_vector() + (m.fuzz * Vec3::random_unit_vector());
                let scattered = Ray::new(&rec.p, &reflected);

                if scattered.direction().dot(&rec.normal) > 0.0 {
                    Some((m.albedo, scattered))
                } else {
                    None
                }
            }

            Dielectric(d) => {
                let attenuation = Color::white();
                let ri = if rec.front_face {
                    1.0 / d.refraction_index
                } else {
                    d.refraction_index
                };

                let unit_dir = r_in.direction().unit_vector();
                let refracted = refract(unit_dir, rec.normal, ri);

                Some((attenuation, Ray::new(&rec.p, &refracted)))
            }
        }
    }
}
