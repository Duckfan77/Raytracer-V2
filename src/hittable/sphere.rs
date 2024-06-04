use std::f64::consts::PI;

use crate::{
    material::Material,
    vec3::{Point3, Vec3},
};

use super::aabb::Aabb;

#[derive(Clone)]
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
        let r_vec = Vec3::new(radius, radius, radius);
        let box0 = Aabb::from_points(center0 - r_vec, center0 + r_vec);
        let box1 = Aabb::from_points(center1 - r_vec, center1 + r_vec);
        let bbox = Aabb::from_boxes(&box0, &box1);

        Self {
            center0,
            move_vec: Some(center1 - center0),
            radius,
            mat: mat.into(),
            bbox,
        }
    }

    pub(super) fn sphere_center(&self, time: f64) -> Point3 {
        match self.move_vec {
            Some(move_vec) => self.center0 + time * move_vec,
            None => self.center0,
        }
    }

    ///
    /// p: a given point on the sphere of radius one, centered at the origin.
    /// Returns: (u,v)
    /// u: returned value [0,1] of angle around the Y axis from X=-1.
    /// v: returned value [0,1] of angle from Y=-1 to Y=+1.
    ///
    /// < 1  0  0> yields (0.50, 0.50)
    /// < 0  1  0> yields (0.50, 1.00)
    /// < 0  0  1> yields (0.25, 0.50)
    /// <-1  0  0> yields (0.00, 0.75)
    /// < 0 -1  0> yields (0.50, 0.00)
    /// < 0  0 -1> yields (0.75, 0.50)
    ///
    pub(super) fn get_sphere_uv(p: Point3) -> (f64, f64) {
        let theta = (-p.y()).acos();
        let phi = f64::atan2(-p.z(), p.x()) + PI;

        let u = phi / (2.0 * PI);
        let v = theta / PI;

        (u, v)
    }
}
