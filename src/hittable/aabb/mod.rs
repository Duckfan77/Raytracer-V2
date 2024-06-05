use std::ops::Add;

use crate::{
    interval::{from_intervals, AabbHelper, Interval, EMPTY},
    ray::Ray,
    vec3::{Point3, Vec3},
};

#[derive(Clone)]
pub struct Aabb {
    x: Interval,
    y: Interval,
    z: Interval,
}

impl Aabb {
    #[allow(dead_code)]
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        let mut out = Self { x, y, z };
        out.pad_to_minimums();
        out
    }

    pub fn empty() -> Self {
        let mut out = Self {
            x: EMPTY,
            y: EMPTY,
            z: EMPTY,
        };
        out.pad_to_minimums();
        out
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

        let mut out = Self { x, y, z };
        out.pad_to_minimums();
        out
    }

    pub fn from_boxes(box0: &Self, box1: &Self) -> Self {
        let x = from_intervals(&box0.x, &box1.x);
        let y = from_intervals(&box0.y, &box1.y);
        let z = from_intervals(&box0.z, &box1.z);

        Self { x, y, z }
    }

    pub fn index(&self, index: usize) -> &Interval {
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

    pub fn longest_axis(&self) -> i32 {
        if self.x.size() > self.y.size() {
            if self.x.size() > self.z.size() {
                0
            } else {
                2
            }
        } else {
            if self.y.size() > self.z.size() {
                1
            } else {
                2
            }
        }
    }

    fn pad_to_minimums(&mut self) {
        const DELTA: f64 = 0.0001;

        if self.x.size() < DELTA {
            self.x = self.x.expand(DELTA);
        }
        if self.y.size() < DELTA {
            self.y = self.y.expand(DELTA);
        }
        if self.z.size() < DELTA {
            self.z = self.z.expand(DELTA);
        }
    }
}

impl Add<Vec3> for Aabb {
    type Output = Aabb;

    fn add(self, rhs: Vec3) -> Self::Output {
        Aabb::new(
            self.x.add(rhs.x()),
            self.y.add(rhs.y()),
            self.z.add(rhs.z()),
        )
    }
}

impl Add<Aabb> for Vec3 {
    type Output = Aabb;

    fn add(self, rhs: Aabb) -> Self::Output {
        rhs + self
    }
}
