use std::sync::Arc;

use crate::vec3::Vec3;

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
