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
