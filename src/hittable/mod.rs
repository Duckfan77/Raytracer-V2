pub mod hittable_list;
pub mod sphere;

use crate::{
    interval::Interval,
    ray::Ray,
    vec3::{Point3, Vec3},
};

///
/// p: Point on the Hittable where the hit occurred
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
    HittableList(hittable_list::HittableList),
}

impl Hittable {
    pub fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
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
                if root <= *ray_t.start() || *ray_t.end() <= root {
                    // root outside range
                    root = (h + sqrtd) / a;
                    if root <= *ray_t.start() || *ray_t.end() <= root {
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
            HittableList(h) => {
                let mut best_so_far = *ray_t.end();
                let mut temp_rec = None;
                for object in h.objects.iter() {
                    if let Some(rec) = object.hit(r, *ray_t.start()..=best_so_far) {
                        best_so_far = rec.t;
                        temp_rec = Some(rec);
                    }
                }
                temp_rec
            }
        }
    }
}
