mod aabb;
pub mod bvh;
pub mod constant_medium;
pub mod hittable_list;
pub mod instance;
pub mod quad;
pub mod sphere;

use std::f64::INFINITY;

use aabb::Aabb;

use crate::{
    interval::{self, Interval},
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
    ConstantMedium(constant_medium::ConstantMedium),
    HittableList(hittable_list::HittableList),
    BvhNode(bvh::BvhNode),
    Translate(instance::Translate),
    YRotate(instance::YRotate),
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

impl From<constant_medium::ConstantMedium> for Hittable {
    fn from(value: constant_medium::ConstantMedium) -> Self {
        Hittable::ConstantMedium(value)
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

impl From<instance::YRotate> for Hittable {
    fn from(value: instance::YRotate) -> Self {
        Hittable::YRotate(value)
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

            ConstantMedium(m) => {
                // This hit function assumes the boundary of m is convex. It WILL NOT WORK if it isn't.

                // Print occasional samples when debugging. To enable, set ENABLE_DEBUG true.
                const ENABLE_DEBUG: bool = false;
                let debugging = ENABLE_DEBUG && rand::random::<f64>() < 0.00001;

                // Get two hits on the boundary, to know min and max locations of hits
                let rec1 = m.boundary.hit(r, interval::UNIVERSE);
                if rec1.is_none() {
                    return None;
                }
                let mut rec1 =
                    rec1.expect("This is safe, because we just handled the None case above");

                let rec2 = m.boundary.hit(r, (rec1.t + 0.0001)..=INFINITY);
                if rec2.is_none() {
                    return None;
                }
                let mut rec2 =
                    rec2.expect("This is safe, because we just handled the None case above");

                if debugging {
                    println!("\nt_min={}, t_max={}", rec1.t, rec2.t);
                }

                // Limit boundary hit positions to locations of interest defined by ray_t
                if rec1.t < *ray_t.start() {
                    rec1.t = *ray_t.start()
                }
                if rec2.t > *ray_t.end() {
                    rec2.t = *ray_t.start()
                }

                // If the second hit is before the first hit, then we didn't hit the boundary
                if rec1.t >= rec2.t {
                    return None;
                }

                // Clip the start time to 0 at minimum, no negative hit locations
                if rec1.t < 0.0 {
                    rec1.t = 0.0
                }

                let ray_length = r.direction().length();
                let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
                let hit_distance = m.neg_inv_density * rand::random::<f64>().ln();

                if hit_distance > distance_inside_boundary {
                    return None;
                }

                let t = rec1.t + hit_distance / ray_length;
                let p = r.at(t);

                if debugging {
                    println!("hit_distance = {hit_distance}\nrec.t = {t}\nrec.p = {p}");
                }

                let normal = Vec3::new(1.0, 0.0, 0.0); // Arbitrary, doesn't make sense for a constant medium
                let front_face = true; // Arbitrary, doesn't make sense for a constant medium
                let mat = m.phase_function.clone();

                Some(HitRecord {
                    t,
                    p,
                    normal,
                    front_face,
                    mat,
                    u: 0.0,
                    v: 0.0,
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

            YRotate(rot) => {
                // Change from world space to object space
                let mut origin = *r.origin();
                let mut direction = *r.direction();

                origin[0] = rot.cos_theta * r.origin()[0] - rot.sin_theta * r.origin()[2];
                origin[2] = rot.sin_theta * r.origin()[0] + rot.cos_theta * r.origin()[2];

                direction[0] = rot.cos_theta * r.direction()[0] - rot.sin_theta * r.direction()[2];
                direction[2] = rot.sin_theta * r.direction()[0] + rot.cos_theta * r.direction()[2];

                let rotated_r = Ray::with_time(origin, direction, r.time());

                if let Some(mut rec) = rot.object.hit(rotated_r, ray_t) {
                    // Change the intersection point from object space to world space
                    let mut p = rec.p;
                    p[0] = rot.cos_theta * rec.p[0] + rot.sin_theta * rec.p[2];
                    p[2] = -rot.sin_theta * rec.p[0] + rot.cos_theta * rec.p[2];
                    rec.p = p;

                    // Change the normal from object space to world space
                    let mut normal = rec.normal;
                    normal[0] = rot.cos_theta * rec.normal[0] + rot.sin_theta * rec.normal[2];
                    normal[2] = -rot.sin_theta * rec.normal[0] + rot.cos_theta * rec.normal[2];
                    rec.normal = normal;

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

            ConstantMedium(m) => m.boundary.bounding_box(),

            HittableList(h) => h.bbox.clone(),

            BvhNode(bvh) => bvh.bbox.clone(),

            Translate(t) => t.bbox.clone(),

            YRotate(y) => y.bbox.clone(),
        }
    }
}
