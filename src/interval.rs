use std::ops::RangeInclusive;

use std::f64::{INFINITY, NEG_INFINITY};

#[allow(dead_code)]
pub const EMPTY: RangeInclusive<f64> = INFINITY..=NEG_INFINITY;
#[allow(dead_code)]
pub const UNIVERSE: RangeInclusive<f64> = NEG_INFINITY..=INFINITY;

pub type Interval = RangeInclusive<f64>;

pub trait Surrounds {
    #[allow(dead_code)]
    fn surrounds(&self, x: f64) -> bool;
}

pub trait Clamp {
    fn clamp(&self, x: f64) -> f64;
}

pub trait AabbHelper {
    #[allow(dead_code)]
    fn expand(&self, delta: f64) -> Self;
    fn size(&self) -> f64;
}

impl Surrounds for RangeInclusive<f64> {
    fn surrounds(&self, x: f64) -> bool {
        *self.start() < x && x < *self.end()
    }
}

impl Clamp for RangeInclusive<f64> {
    fn clamp(&self, x: f64) -> f64 {
        if x < *self.start() {
            *self.start()
        } else if x > *self.end() {
            *self.end()
        } else {
            x
        }
    }
}

impl AabbHelper for RangeInclusive<f64> {
    fn expand(&self, delta: f64) -> Self {
        let padding = delta / 2.0;
        (self.start() - padding)..=(self.end() + padding)
    }

    fn size(&self) -> f64 {
        self.end() - self.start()
    }
}

pub fn from_intervals(a: &Interval, b: &Interval) -> Interval {
    let start = a.start().min(*b.start());
    let end = a.end().max(*b.end());
    start..=end
}
