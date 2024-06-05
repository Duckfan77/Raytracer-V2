use crate::{
    material::Material,
    vec3::{Point3, Vec3},
};

use super::aabb::Aabb;

#[derive(Clone)]
pub struct Quad {
    pub(super) q: Point3,
    pub(super) u: Vec3,
    pub(super) v: Vec3,
    pub(super) bbox: Aabb,
    pub(super) mat: Material,
    pub(super) normal: Vec3, // (A, B, B) constants for the plane containing the Quad
    #[allow(non_snake_case)]
    pub(super) D: f64, // remaining constant for the plane containing the Quad
}

impl Quad {
    pub fn new(q: Point3, u: Point3, v: Point3, mat: Material) -> Self {
        let n = u.cross(v);
        let normal = n.unit_vector();
        let d = normal.dot(q);

        Self {
            q,
            u,
            v,
            mat,
            bbox: set_bounding_box(q, u, v),
            normal,
            D: d,
        }
    }
}

fn set_bounding_box(q: Point3, u: Point3, v: Point3) -> Aabb {
    let bbox_diagonal1 = Aabb::from_points(q, q + u + v);
    let bbox_diagonal2 = Aabb::from_points(q + u, q + v);
    Aabb::from_boxes(&bbox_diagonal1, &bbox_diagonal2)
}
