use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub},
};

use once_cell::sync::Lazy;
use rand::{
    distributions::{Distribution, Uniform},
    random,
    rngs::ThreadRng,
};

pub type Point3 = Vec3;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Vec3(f64, f64, f64);

static UNIT_SPHERE_DIST: Lazy<Uniform<f64>> = Lazy::new(|| Uniform::from(-1.0..1.0));

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Self(e0, e1, e2)
    }

    pub fn random() -> Self {
        Self(random(), random(), random())
    }

    pub fn random_range(dist: &Uniform<f64>, rng: &mut ThreadRng) -> Self {
        Self(dist.sample(rng), dist.sample(rng), dist.sample(rng))
    }

    pub fn random_dist(dist: &Uniform<f64>, rng: &mut ThreadRng) -> Self {
        Self(dist.sample(rng), dist.sample(rng), dist.sample(rng))
    }

    pub fn random_in_unit_sphere() -> Self {
        let mut rng = rand::thread_rng();
        loop {
            let p = Vec3::random_dist(&UNIT_SPHERE_DIST, &mut rng);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_unit_vector() -> Self {
        Vec3::random_in_unit_sphere().unit_vector()
    }

    pub fn random_on_hemisphere(normal: Vec3) -> Self {
        let on_sphere = Vec3::random_unit_vector();
        if on_sphere.dot(normal) > 0.0 {
            // In the same hemisphere as the normal
            on_sphere
        } else {
            -on_sphere
        }
    }

    pub fn random_in_unit_disk() -> Self {
        let mut rng = rand::thread_rng();
        loop {
            let p = Vec3::new(
                UNIT_SPHERE_DIST.sample(&mut rng),
                UNIT_SPHERE_DIST.sample(&mut rng),
                0.0,
            );
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    ///
    /// Reflects self off the plane defined by `normal`
    ///
    pub fn reflect(&self, normal: Vec3) -> Self {
        *self - 2.0 * self.dot(normal) * normal
    }

    pub fn x(&self) -> f64 {
        self.0
    }

    pub fn y(&self) -> f64 {
        self.1
    }

    pub fn z(&self) -> f64 {
        self.2
    }

    pub fn length_squared(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(&self, other: Self) -> f64 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    pub fn cross(&self, other: Self) -> Self {
        Self(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        )
    }

    pub fn unit_vector(&self) -> Vec3 {
        *self / self.length()
    }

    pub fn near_zero(&self) -> bool {
        const NEAR_ZERO_THRESHOLD: f64 = 1e-8;
        self.0.abs() < NEAR_ZERO_THRESHOLD
            && self.1.abs() < NEAR_ZERO_THRESHOLD
            && self.2.abs() < NEAR_ZERO_THRESHOLD
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3(-self.0, -self.1, -self.2)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs;
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x:{} y:{} z:{}", self.0, self.1, self.2)
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        self + -rhs
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}
