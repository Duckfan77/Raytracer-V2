pub mod sphere;

use crate::{
    ray::Ray,
    vec3::{Point3, Vec3},
};

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
}

#[non_exhaustive]
pub enum Hittable {
    Sphere(sphere::Sphere),
}

impl Hittable {
    pub fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
        use Hittable::*;
        match self {
            Sphere(s) => {
                let oc = s.center - *r.origin();
                let a = r.direction().length_squared();
                let h = r.direction().dot(&oc);
                let c = oc.length_squared() - s.radius * s.radius;
                let discriminant = h * h - a * c;

                if discriminant < 0.0 {
                    return None;
                }

                let sqrtd = discriminant.sqrt();

                // Find the nearest root in the acceptable range
                let mut root = (h - sqrtd) / a;
                if root <= ray_tmin || ray_tmax <= root {
                    // root outside range
                    root = (h + sqrtd) / a;
                    if root <= ray_tmin || ray_tmax <= root {
                        // root outside range
                        return None;
                    }
                }

                {
                    let t = root;
                    let p = r.at(t);
                    let normal = (p - s.center) / s.radius;

                    Some(HitRecord { t, p, normal })
                }
            }
        }
    }
}
