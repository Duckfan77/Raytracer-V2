use crate::vec3::Vec3;

///
/// uv: unit vector of the incoming ray direction
/// n: unit normal vector of the surface
/// etai_over_etat: ratio of the external index of refraction to the internal index of refraction
///
/// Returns the direction of the refracted ray
///
pub(super) fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = -uv.dot(&n).min(1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * n;
    r_out_perp + r_out_parallel
}

pub(super) fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
    // Use Schlick's approximation for reflectance
    let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

pub use refractive_indices::*;

#[allow(dead_code)]
mod refractive_indices {
    pub static RI_AIR: f64 = 1.0;
    pub static RI_GLASS: f64 = 1.5;
    pub static RI_WATER: f64 = 1.333;
    pub static RI_DIAMOND: f64 = 2.417;
}
pub struct Dielectric {
    ///
    /// Refractive index in vacuum or air, or ratio of the material's refractive index over the
    /// refractive index of the enclosing media
    ///
    pub(super) refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }
}
