use crate::{
    interval::Interval,
    material::Material,
    vec3::{Point3, Vec3},
};

use super::aabb::Aabb;

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
}

fn set_bounding_box(q: Point3, u: Point3, v: Point3) -> Aabb {
    let bbox_diagonal1 = Aabb::from_points(q, q + u + v);
    let bbox_diagonal2 = Aabb::from_points(q + u, q + v);
    Aabb::from_boxes(&bbox_diagonal1, &bbox_diagonal2)
}
