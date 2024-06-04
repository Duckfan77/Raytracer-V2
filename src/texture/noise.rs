use super::perlin::PerlinNoise;

#[derive(Clone)]
pub struct Noise {
    pub(super) noise: PerlinNoise,
}

impl Noise {
    pub fn new() -> Self {
        Self {
            noise: PerlinNoise::new(),
        }
    }
}
