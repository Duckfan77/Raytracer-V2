use crate::{
    interval::Interval,
    material::Material,
    vec3::{Point3, Vec3},
};

use super::{aabb::Aabb, hittable_list::HittableList};

#[derive(Clone)]
pub struct Quad {
    pub(super) q: Point3, // One vertex of the quad
    pub(super) u: Vec3,   // vector from q to first adjacent vertex
    pub(super) v: Vec3,   // vector from q to second adjacent vertex
    pub(super) w: Vec3,   // Constant used in intersection testing
    pub(super) bbox: Aabb,
    pub(super) mat: Material,
    pub(super) normal: Vec3, // (A, B, B) constants for the plane containing the Quad
    pub(super) d: f64,       // remaining constant D for the plane containing the Quad
}

impl Quad {
    pub fn new(q: Point3, u: Point3, v: Point3, mat: impl Into<Material>) -> Self {
        let n = u.cross(v);
        let normal = n.unit_vector();
        let d = normal.dot(q);
        let w = n / n.dot(n);

        Self {
            q,
            u,
            v,
            w,
            mat: mat.into(),
            bbox: set_bounding_box(q, u, v),
            normal,
            d,
        }
    }

    pub(super) fn quad_uv(&self, intersection: Vec3) -> Option<(f64, f64)> {
        const UNIT_INTERVAL: Interval = 0.0..=1.0;

        let planar_hit_point = intersection - self.q;
        let alpha = self.w.dot(Vec3::cross(&planar_hit_point, self.v));
        let beta = self.w.dot(Vec3::cross(&self.u, planar_hit_point));

        if !UNIT_INTERVAL.contains(&alpha) || !UNIT_INTERVAL.contains(&beta) {
            None
        } else {
            Some((alpha, beta))
        }
    }

    pub fn new_box(a: Point3, b: Point3, mat: impl Into<Material>) -> HittableList {
        let mut sides = HittableList::new();

        let min = Point3::new(
            f64::min(a.x(), b.x()),
            f64::min(a.y(), b.y()),
            f64::min(a.z(), b.z()),
        );
        let max = Point3::new(
            f64::max(a.x(), b.x()),
            f64::max(a.y(), b.y()),
            f64::max(a.z(), b.z()),
        );

        let dx = Vec3::new(max.x() - min.x(), 0.0, 0.0);
        let dy = Vec3::new(0.0, max.y() - min.y(), 0.0);
        let dz = Vec3::new(0.0, 0.0, max.z() - min.z());

        let mat = mat.into();

        sides.add(Quad::new(
            // Front
            Point3::new(min.x(), min.y(), max.z()),
            dx,
            dy,
            mat.clone(),
        ));
        sides.add(Quad::new(
            // Right
            Point3::new(max.x(), min.y(), max.z()),
            -dz,
            dy,
            mat.clone(),
        ));
        sides.add(Quad::new(
            // Back
            Point3::new(max.x(), min.y(), min.z()),
            -dx,
            dy,
            mat.clone(),
        ));
        sides.add(Quad::new(
            // Left
            Point3::new(min.x(), min.y(), min.z()),
            dz,
            dy,
            mat.clone(),
        ));
        sides.add(Quad::new(
            // Top
            Point3::new(min.x(), max.y(), max.z()),
            dx,
            -dz,
            mat.clone(),
        ));
        sides.add(Quad::new(
            Point3::new(min.x(), min.y(), min.z()),
            dx,
            dz,
            mat,
        ));

        sides
    }
}

fn set_bounding_box(q: Point3, u: Point3, v: Point3) -> Aabb {
    let bbox_diagonal1 = Aabb::from_points(q, q + u + v);
    let bbox_diagonal2 = Aabb::from_points(q + u, q + v);
    Aabb::from_boxes(&bbox_diagonal1, &bbox_diagonal2)
}
