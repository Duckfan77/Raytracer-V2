use super::perlin::PerlinNoise;

#[derive(Clone)]
pub struct Noise {
    pub(super) noise: PerlinNoise,
    pub(super) scale: f64,
}

impl Noise {
    pub fn new(scale: f64) -> Self {
        Self {
            noise: PerlinNoise::new(),
            scale,
        }
    }
}

#[derive(Clone)]
pub struct TurbNoise {
    pub(super) noise: PerlinNoise,
    pub(super) scale: f64,
    pub(super) depth: u32,
}

impl TurbNoise {
    pub fn new(scale: f64, depth: u32) -> Self {
        Self {
            noise: PerlinNoise::new(),
            scale,
            depth,
        }
    }
}
