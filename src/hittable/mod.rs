use crate::{
    ray::Ray,
    vec3::{Point3, Vec3},
};

pub struct HitRecord {
    p: Point3,
    normal: Vec3,
    t: f64,
}

#[non_exhaustive]
pub enum Hittable {}

impl Hittable {
    pub fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
        use Hittable::*;
        match self {
            _ => None,
        }
    }
}
