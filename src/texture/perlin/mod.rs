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
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();
        let u = u * u * (3.0 - 2.0 * u);
        let v = v * v * (3.0 - 2.0 * v);
        let w = w * w * (3.0 - 2.0 * w);

        let i = p.x().floor() as i32;
        let j = p.y().floor() as i32;
        let k = p.z().floor() as i32;

        let mut c = [[[0.0; 2]; 2]; 2];
        for di in 0i32..2 {
            for dj in 0i32..2 {
                for dk in 0i32..2 {
                    c[di as usize][dj as usize][dk as usize] = self.rand_float[self.perm_x
                        [((i + di) & 255) as usize]
                        ^ self.perm_y[((j + dj) & 255) as usize]
                        ^ self.perm_z[((k + dk) & 255) as usize]]
                }
            }
        }

        trilinear_interpolation(c, u, v, w)
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

fn trilinear_interpolation(c: [[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
    let mut acc = 0.0;

    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let fi = i as f64;
                let fj = j as f64;
                let fk = k as f64;
                acc += (fi * u + (1.0 - fi) * (1.0 - u))
                    * (fj * v + (1.0 - fj) * (1.0 - v))
                    * (fk * w + (1.0 - fk) * (1.0 - w))
                    * c[i][j][k];
            }
        }
    }

    acc
}
