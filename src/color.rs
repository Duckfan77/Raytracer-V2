use std::{
    fmt::Display,
    iter::Sum,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub},
};

use crate::interval::{Clamp, Interval};
use image::{Rgb, RgbImage};
use rand::{
    distributions::{Distribution, Uniform},
    random,
    rngs::ThreadRng,
};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Color(f64, f64, f64);

impl Color {
    pub fn black() -> Self {
        Self(0.0, 0.0, 0.0)
    }

    pub fn white() -> Self {
        Self(1.0, 1.0, 1.0)
    }

    pub fn half_grey() -> Self {
        Self(0.5, 0.5, 0.5)
    }

    pub fn random() -> Self {
        Self(random(), random(), random())
    }

    pub fn random_range(dist: Uniform<f64>, rng: &mut ThreadRng) -> Self {
        Self(dist.sample(rng), dist.sample(rng), dist.sample(rng))
    }

    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Self(e0, e1, e2)
    }

    pub fn r(&self) -> f64 {
        self.0
    }

    pub fn g(&self) -> f64 {
        self.1
    }

    pub fn b(&self) -> f64 {
        self.2
    }
}

fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        linear_component.sqrt()
    } else {
        0.0
    }
}

pub fn write_color(img: &mut RgbImage, color: &Color, u: u32, v: u32) {
    img.put_pixel(u, v, color.into());
}

pub fn write_row(img: &mut RgbImage, row: &[Color], v: u32) {
    for (u, color) in row.iter().enumerate() {
        write_color(img, color, u as u32, v);
    }
}

impl From<&Color> for Rgb<u8> {
    fn from(value: &Color) -> Self {
        const SCALE_FACTOR: f64 = 256.0;
        const INTENSITY: Interval = 0.0..=0.999;

        let r = linear_to_gamma(value.r());
        let g = linear_to_gamma(value.g());
        let b = linear_to_gamma(value.b());

        let ir = (SCALE_FACTOR * INTENSITY.clamp(r)) as u8;
        let ig = (SCALE_FACTOR * INTENSITY.clamp(g)) as u8;
        let ib = (SCALE_FACTOR * INTENSITY.clamp(b)) as u8;

        Self([ir, ig, ib])
    }
}

impl From<crate::vec3::Vec3> for Color {
    fn from(value: crate::vec3::Vec3) -> Self {
        Self(value.x(), value.y(), value.z())
    }
}

impl Sum for Color {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Color::black(), |a, b| a + b)
    }
}

impl Neg for Color {
    type Output = Color;

    fn neg(self) -> Self::Output {
        Color(-self.0, -self.1, -self.2)
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl MulAssign<f64> for Color {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
    }
}

impl DivAssign<f64> for Color {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs;
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, rhs: Self) -> Self::Output {
        Color(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Sub for Color {
    type Output = Color;

    fn sub(self, rhs: Self) -> Self::Output {
        self + -rhs
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, rhs: Self) -> Self::Output {
        Color(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Self::Output {
        Color(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        rhs * self
    }
}

impl Div<f64> for Color {
    type Output = Color;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "r:{} g:{} b:{}", self.0, self.1, self.2)
    }
}
