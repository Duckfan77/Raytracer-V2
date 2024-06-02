use crate::vec3::{Point3, Vec3};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(origin: &Point3, direction: &Vec3) -> Self {
        Self {
            orig: *origin,
            dir: *direction,
        }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig + t * self.dir
    }

    pub fn origin(&self) -> &Point3 {
        &self.orig
    }

    pub fn direction(&self) -> &Vec3 {
        &self.dir
    }

    #[allow(dead_code)]
    pub fn origin_mut(&mut self) -> &mut Point3 {
        &mut self.orig
    }

    #[allow(dead_code)]
    pub fn direction_mut(&mut self) -> &mut Point3 {
        &mut self.dir
    }
}
