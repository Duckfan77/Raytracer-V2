use std::{
    f64::{INFINITY, NEG_INFINITY},
    sync::Arc,
};

use crate::vec3::{Point3, Vec3};

use super::{aabb::Aabb, Hittable};

#[derive(Clone)]
pub struct Translate {
    pub(super) object: Arc<Hittable>,
    pub(super) offset: Vec3,
    pub(super) bbox: Aabb,
}

impl Translate {
    pub fn new(object: impl Into<Hittable>, offset: Vec3) -> Self {
        let object = Arc::new(object.into());
        let bbox = object.bounding_box() + offset;
        Self {
            object,
            offset,
            bbox,
        }
    }

    #[allow(dead_code)]
    pub fn from_arc(object: Arc<Hittable>, offset: Vec3) -> Self {
        let bbox = object.bounding_box() + offset;
        Self {
            object,
            offset,
            bbox,
        }
    }
}

#[derive(Clone)]
pub struct YRotate {
    pub(super) object: Arc<Hittable>,
    pub(super) sin_theta: f64,
    pub(super) cos_theta: f64,
    pub(super) bbox: Aabb,
}

impl YRotate {
    pub fn new(object: impl Into<Hittable>, angle: f64) -> Self {
        Self::from_arc(Arc::new(object.into()), angle)
    }

    pub fn from_arc(object: Arc<Hittable>, angle: f64) -> Self {
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox: Aabb = object.bounding_box();

        let mut min = Point3::new(INFINITY, INFINITY, INFINITY);
        let mut max = Point3::new(NEG_INFINITY, NEG_INFINITY, NEG_INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let fi = i as f64;
                    let fj = j as f64;
                    let fk = k as f64;

                    let x = fi * bbox.index(0).end() + (1.0 - fi) * bbox.index(0).start();
                    let y = fj * bbox.index(1).end() + (1.0 - fj) * bbox.index(1).start();
                    let z = fk * bbox.index(2).end() + (1.0 - fk) * bbox.index(2).start();

                    let new_x = cos_theta * x + sin_theta * z;
                    let new_z = -sin_theta * x + cos_theta * z;

                    let tester = Vec3::new(new_x, y, new_z);

                    for c in 0..3 {
                        min[c] = f64::min(min[c], tester[c]);
                        max[c] = f64::max(max[c], tester[c])
                    }
                }
            }
        }

        Self {
            object,
            sin_theta,
            cos_theta,
            bbox,
        }
    }
}
