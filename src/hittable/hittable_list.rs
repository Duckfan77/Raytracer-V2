use super::{aabb::Aabb, Hittable};

#[derive(Clone)]
pub struct HittableList {
    pub(super) objects: Vec<Hittable>,
    pub(super) bbox: Aabb,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            bbox: Aabb::empty(),
        }
    }

    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: impl Into<Hittable>) {
        let object = object.into();
        self.bbox = Aabb::from_boxes(&self.bbox, &object.bounding_box());
        self.objects.push(object);
    }
}

impl FromIterator<Hittable> for HittableList {
    fn from_iter<T: IntoIterator<Item = Hittable>>(iter: T) -> Self {
        let objects: Vec<Hittable> = iter.into_iter().collect();
        let bbox = objects.iter().fold(Aabb::empty(), |bbox, object| {
            Aabb::from_boxes(&bbox, &object.bounding_box())
        });
        Self { objects, bbox }
    }
}
