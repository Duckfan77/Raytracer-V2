use crate::{
    material::Material,
    vec3::{Point3, Vec3},
};

use super::aabb::Aabb;

pub struct Sphere {
    pub(super) center0: Point3,
    pub(super) move_vec: Option<Vec3>,
    pub(super) radius: f64,
    pub(super) mat: Material,
    pub(super) bbox: Aabb,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat: impl Into<Material>) -> Self {
        let r_vec = Vec3::new(radius, radius, radius);
        let bbox = Aabb::from_points(center - r_vec, center + r_vec);
        Self {
            center0: center,
            move_vec: None,
            radius,
            mat: mat.into(),
            bbox,
        }
    }

    pub fn new_moving(
        center0: Point3,
        center1: Point3,
        radius: f64,
        mat: impl Into<Material>,
    ) -> Self {
        unimplemented!();
        /*Self {
            center0,
            move_vec: Some(center1 - center0),
            radius,
            mat: mat.into(),
        }*/
    }

    pub(super) fn sphere_center(&self, time: f64) -> Point3 {
        match self.move_vec {
            Some(move_vec) => self.center0 + time * move_vec,
            None => self.center0,
        }
    }
}
