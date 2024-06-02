pub mod sphere;

use crate::{
    ray::Ray,
    vec3::{Point3, Vec3},
};

///
/// p: Point on the Hittable where the hit occured
/// normal: The outward facing unit normal vector at the location of the hit
/// t: the time of the hit
/// front_face: true when the ray faces opposite the outward facing normal, false otherwise
///
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    fn get_face_normal(r: &Ray, outward_normal: &Vec3) -> (bool, Vec3) {
        let front_face = r.direction().dot(outward_normal) < 0.0;
        let normal = if front_face {
            *outward_normal
        } else {
            -*outward_normal
        };

        (front_face, normal)
    }
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
                    let outward_normal = (p - s.center) / s.radius;
                    let (front_face, normal) = HitRecord::get_face_normal(r, &outward_normal);

                    Some(HitRecord {
                        t,
                        p,
                        normal,
                        front_face,
                    })
                }
            }
        }
    }
}
