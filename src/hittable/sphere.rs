use crate::vec3::Point3;

pub struct Sphere {
    pub(super) center: Point3,
    pub(super) radius: f64,
}

impl Sphere {
    pub fn new(center: &Point3, radius: f64) -> Self {
        Self {
            center: *center,
            radius,
        }
    }
}
