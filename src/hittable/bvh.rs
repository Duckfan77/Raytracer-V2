use super::{aabb::Aabb, hittable_list::HittableList, Hittable};

pub struct BvhNode {
    pub(super) left: Box<Hittable>,
    pub(super) right: Box<Hittable>,
    pub(super) bbox: Aabb,
}

impl BvhNode {
    pub fn from_list(list: HittableList) -> Self {
        let len = list.objects.len();
        BvhNode::new(list.objects, 0, len)
    }

    pub fn new(objects: Vec<Hittable>, start: usize, end: usize) -> Self {
        unimplemented!()
    }
}
