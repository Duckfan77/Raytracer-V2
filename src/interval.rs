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
    fn overlaps(&self, other: Self) -> bool;
    fn expand(&self, delta: f64) -> Self;
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
    fn overlaps(&self, other: Self) -> bool {
        let t_min = f64::max(*self.start(), *other.start());
        let t_max = f64::min(*self.end(), *other.end());
        t_min < t_max
    }

    fn expand(&self, delta: f64) -> Self {
        let padding = delta / 2.0;
        (self.start() - padding)..=(self.end() + padding)
    }
}
