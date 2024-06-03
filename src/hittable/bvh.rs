use std::cmp::Ordering;

use super::{aabb::Aabb, hittable_list::HittableList, Hittable};

#[derive(Clone)]
pub struct BvhNode {
    pub(super) left: Box<Hittable>,
    pub(super) right: Box<Hittable>,
    pub(super) bbox: Aabb,
}

impl BvhNode {
    pub fn from_list(list: HittableList) -> Self {
        BvhNode::new(list.objects)
    }

    pub fn new(mut objects: Vec<Hittable>) -> Self {
        let mut bbox = Aabb::empty();
        for o in objects.iter() {
            bbox = Aabb::from_boxes(&bbox, &o.bounding_box());
        }

        let axis = bbox.longest_axis();

        let comparator = match axis {
            0 => box_x_compare,
            1 => box_y_compare,
            2 => box_z_compare,
            _ => unreachable!(),
        };

        let object_span = objects.len();

        let left;
        let right;
        if object_span == 1 {
            left = objects[0].clone();
            right = objects[0].clone();
        } else if object_span == 2 {
            left = objects[0].clone();
            right = objects[1].clone();
        } else {
            objects.sort_unstable_by(comparator);

            let mid = object_span / 2;

            let right_list = objects.split_off(mid);
            let left_list = objects;

            left = BvhNode::new(left_list).into();
            right = BvhNode::new(right_list).into();
        }

        let bbox = Aabb::from_boxes(&left.bounding_box(), &right.bounding_box());

        Self {
            left: Box::new(left),
            right: Box::new(right),
            bbox,
        }
    }
}

fn box_compare(a: &Hittable, b: &Hittable, axis_index: usize) -> Ordering {
    let a_axis_interval = a.bounding_box().index(axis_index).clone();
    let b_axis_interval = b.bounding_box().index(axis_index).clone();

    a_axis_interval
        .start()
        .partial_cmp(b_axis_interval.start())
        .unwrap_or(Ordering::Less)
}

fn box_x_compare(a: &Hittable, b: &Hittable) -> Ordering {
    box_compare(a, b, 0)
}

fn box_y_compare(a: &Hittable, b: &Hittable) -> Ordering {
    box_compare(a, b, 1)
}

fn box_z_compare(a: &Hittable, b: &Hittable) -> Ordering {
    box_compare(a, b, 2)
}
