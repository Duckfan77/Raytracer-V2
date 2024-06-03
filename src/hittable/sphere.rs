use std::sync::Arc;

use crate::{material::Material, vec3::Point3};

pub struct Sphere {
    pub(super) center: Point3,
    pub(super) radius: f64,
    pub(super) mat: Arc<Material>,
}

impl Sphere {
    pub fn new(center: &Point3, radius: f64, mat: impl Into<Arc<Material>>) -> Self {
        Self {
            center: *center,
            radius,
            mat: mat.into(),
        }
    }
}
