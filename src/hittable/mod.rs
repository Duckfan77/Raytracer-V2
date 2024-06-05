mod aabb;
pub mod bvh;
pub mod hittable_list;
pub mod instance;
pub mod quad;
pub mod sphere;

use aabb::Aabb;

use crate::{
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::{Point3, Vec3},
};

///
/// p: Point on the Hittable where the hit occurred
/// normal: The outward facing unit normal vector at the location of the hit
/// mat: The material of the object hit
/// t: the time of the hit
/// front_face: true when the ray faces opposite the outward facing normal, false otherwise
///
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: Material,
    pub t: f64, // location of hit along ray
    pub u: f64, // location of hit on surface
    pub v: f64, // location of hit on surface
    pub front_face: bool,
}

impl HitRecord {
    fn get_face_normal(r: Ray, outward_normal: Vec3) -> (bool, Vec3) {
        let front_face = r.direction().dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        (front_face, normal)
    }
}

#[non_exhaustive]
#[derive(Clone)]
pub enum Hittable {
    Sphere(sphere::Sphere),
    Quad(quad::Quad),
    HittableList(hittable_list::HittableList),
    BvhNode(bvh::BvhNode),
    Translate(instance::Translate),
}

impl From<sphere::Sphere> for Hittable {
    fn from(value: sphere::Sphere) -> Self {
        Hittable::Sphere(value)
    }
}

impl From<quad::Quad> for Hittable {
    fn from(value: quad::Quad) -> Self {
        Hittable::Quad(value)
    }
}

impl From<hittable_list::HittableList> for Hittable {
    fn from(value: hittable_list::HittableList) -> Self {
        Hittable::HittableList(value)
    }
}

impl From<bvh::BvhNode> for Hittable {
    fn from(value: bvh::BvhNode) -> Self {
        Hittable::BvhNode(value)
    }
}

impl From<instance::Translate> for Hittable {
    fn from(value: instance::Translate) -> Self {
        Hittable::Translate(value)
    }
}

impl Hittable {
    pub fn hit(&self, r: Ray, ray_t: Interval) -> Option<HitRecord> {
        use Hittable::*;
        match self {
            Sphere(s) => {
                let center = s.sphere_center(r.time());
                let oc = center - *r.origin();
                let a = r.direction().length_squared();
                let h = r.direction().dot(oc);
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
                    let mat = s.mat.clone();
                    let outward_normal = (p - center) / s.radius;
                    let (front_face, normal) = HitRecord::get_face_normal(r, outward_normal);
                    let (u, v) = sphere::Sphere::get_sphere_uv(outward_normal);

                    Some(HitRecord {
                        t,
                        p,
                        mat,
                        normal,
                        front_face,
                        u,
                        v,
                    })
                }
            }

            Quad(q) => {
                let denom = q.normal.dot(*r.direction());

                // No hit if the ray is parallel to the plane.
                const NEAR_ZERO_THRESHOLD: f64 = 1e-8;
                if denom.abs() < NEAR_ZERO_THRESHOLD {
                    return None;
                }

                // Return None if hit point parameter t is outside the ray interval
                let t = (q.d - q.normal.dot(*r.origin())) / denom;
                if !ray_t.contains(&t) {
                    return None;
                }

                let p = r.at(t);
                let uv = q.quad_uv(p);
                if uv.is_none() {
                    return None;
                }
                let (u, v) = uv.expect("This is always safe, we just handled the None case above");

                let mat = q.mat.clone();
                let (front_face, normal) = HitRecord::get_face_normal(r, q.normal);

                Some(HitRecord {
                    p,
                    normal,
                    mat,
                    t,
                    u,
                    v,
                    front_face,
                })
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

            BvhNode(bvh) => {
                if !bvh.bbox.hit(r, ray_t.clone()) {
                    return None;
                }

                let mut temp_rec = bvh.left.hit(r, ray_t.clone());
                let interval = if let Some(rec) = &temp_rec {
                    *ray_t.start()..=rec.t
                } else {
                    ray_t
                };
                temp_rec = if let Some(rec) = bvh.right.hit(r, interval) {
                    Some(rec)
                } else {
                    temp_rec
                };

                temp_rec
            }

            Translate(t) => {
                // Move the ray backwards by the offset
                let offset_r = Ray::with_time(*r.origin() - t.offset, *r.direction(), r.time());

                // Determine whether an intersection exists along the offset ray
                if let Some(mut rec) = t.object.hit(offset_r, ray_t) {
                    rec.p += t.offset;
                    Some(rec)
                } else {
                    None
                }
            }
        }
    }

    pub fn bounding_box(&self) -> Aabb {
        use Hittable::*;
        match self {
            Sphere(s) => s.bbox.clone(),

            Quad(q) => q.bbox.clone(),

            HittableList(h) => h.bbox.clone(),

            BvhNode(bvh) => bvh.bbox.clone(),

            Translate(t) => t.bbox.clone(),
        }
    }
}
