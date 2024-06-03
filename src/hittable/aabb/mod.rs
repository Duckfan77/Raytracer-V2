use crate::{interval::Interval, ray::Ray, vec3::Point3};

pub struct Aabb {
    x: Interval,
    y: Interval,
    z: Interval,
}

impl Aabb {
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        Self { x, y, z }
    }

    pub fn from_points(a: Point3, b: Point3) -> Self {
        let x = if a.x() <= b.x() {
            a.x()..=b.x()
        } else {
            b.x()..=a.x()
        };

        let y = if a.y() <= b.y() {
            a.y()..=b.y()
        } else {
            b.y()..=a.y()
        };

        let z = if a.z() <= b.z() {
            a.z()..=b.z()
        } else {
            b.z()..=a.z()
        };

        Self { x, y, z }
    }

    fn index(&self, index: usize) -> &Interval {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Used unknown index value to index into Aabb: {}", index),
        }
    }

    pub fn hit(&self, r: Ray, ray_t: Interval) -> bool {
        let ray_orig = r.origin();
        let ray_dir = r.direction();

        for axis in 0..3 {
            let ax = self.index(axis);
            let adinv = 1.0 / ray_dir[axis];

            let t0 = (ax.start() - ray_orig[axis]) * adinv;
            let t1 = (ax.end() - ray_orig[axis]) * adinv;

            let mut ray_min = *ray_t.start();
            let mut ray_max = *ray_t.end();
            if t0 < t1 {
                if t0 > ray_min {
                    ray_min = t0;
                }
                if t1 < ray_max {
                    ray_max = t1;
                }
            } else {
                if t1 > ray_min {
                    ray_min = t1;
                }
                if t0 < ray_max {
                    ray_max = t0
                }
            }

            if ray_max <= ray_min {
                return false;
            }
        }

        true
    }
}
