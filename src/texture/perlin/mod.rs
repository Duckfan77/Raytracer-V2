use rand::Rng;

use crate::vec3::Point3;

#[derive(Clone)]
pub struct PerlinNoise {
    rand_float: Vec<f64>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

const POINT_COUNT: usize = 256;

impl PerlinNoise {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut rand_float: Vec<f64> = Vec::with_capacity(POINT_COUNT);
        for _ in 0..POINT_COUNT {
            rand_float.push(rng.gen());
        }

        let perm_x = perlin_generate_perm();
        let perm_y = perlin_generate_perm();
        let perm_z = perlin_generate_perm();

        Self {
            rand_float,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn noise(&self, p: Point3) -> f64 {
        let i = (4.0 * p.x()) as usize & 255;
        let j = (4.0 * p.y()) as usize & 255;
        let k = (4.0 * p.z()) as usize & 255;

        self.rand_float[self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]]
    }
}

fn perlin_generate_perm() -> Vec<usize> {
    let mut p: Vec<usize> = (0..POINT_COUNT).into_iter().collect();

    permute(&mut p, POINT_COUNT);

    p
}

fn permute(p: &mut [usize], n: usize) {
    for i in (1..(n - 1)).rev() {
        let target = rand::thread_rng().gen_range(0..=i);
        let tmp = p[i];
        p[i] = p[target];
        p[target] = tmp
    }
}
