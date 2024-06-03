use crate::{
    material::Material,
    vec3::{Point3, Vec3},
};

pub struct Sphere {
    pub(super) center0: Point3,
    pub(super) move_vec: Option<Vec3>,
    pub(super) radius: f64,
    pub(super) mat: Material,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat: impl Into<Material>) -> Self {
        Self {
            center0: center,
            move_vec: None,
            radius,
            mat: mat.into(),
        }
    }

    pub fn new_moving(
        center: Point3,
        radius: f64,
        movement_vec: Vec3,
        mat: impl Into<Material>,
    ) -> Self {
        Self {
            center0: center,
            move_vec: Some(movement_vec),
            radius,
            mat: mat.into(),
        }
    }

    pub(super) fn sphere_center(&self, time: f64) -> Point3 {
        match self.move_vec {
            Some(move_vec) => self.center0 + time * move_vec,
            None => self.center0,
        }
    }
}
