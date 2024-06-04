use rand::{distributions::Uniform, Rng};

use crate::vec3::{Point3, Vec3};

#[derive(Clone)]
pub struct PerlinNoise {
    rand_vec: Vec<Vec3>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

const POINT_COUNT: usize = 256;

impl PerlinNoise {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut rand_vec: Vec<Vec3> = Vec::with_capacity(POINT_COUNT);
        let rand_vec_dist = Uniform::from(-1.0..=1.0);
        for _ in 0..POINT_COUNT {
            rand_vec.push(Vec3::random_dist(&rand_vec_dist, &mut rng).unit_vector());
        }

        let perm_x = perlin_generate_perm();
        let perm_y = perlin_generate_perm();
        let perm_z = perlin_generate_perm();

        Self {
            rand_vec,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn noise(&self, p: Point3) -> f64 {
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();

        let i = p.x().floor() as i32;
        let j = p.y().floor() as i32;
        let k = p.z().floor() as i32;

        let mut c = [[[Vec3::new(0.0, 0.0, 0.0); 2]; 2]; 2];
        for di in 0i32..2 {
            for dj in 0i32..2 {
                for dk in 0i32..2 {
                    c[di as usize][dj as usize][dk as usize] = self.rand_vec[self.perm_x
                        [((i + di) & 255) as usize]
                        ^ self.perm_y[((j + dj) & 255) as usize]
                        ^ self.perm_z[((k + dk) & 255) as usize]]
                }
            }
        }

        perlin_interpolation(c, u, v, w)
    }

    pub fn turb(&self, p: Point3, depth: u32) -> f64 {
        let mut acc = 0.0;
        let mut temp_p = p;
        let mut weight = 1.0;

        for _ in 0..depth {
            acc += weight * self.noise(temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }

        acc.abs()
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

fn perlin_interpolation(c: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
    let uu = u * u * (3.0 - 2.0 * u);
    let vv = v * v * (3.0 - 2.0 * v);
    let ww = w * w * (3.0 - 2.0 * w);

    let mut acc = 0.0;

    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let fi = i as f64;
                let fj = j as f64;
                let fk = k as f64;
                let weight_v = Vec3::new(u - fi, v - fj, w - fk);
                acc += (fi * uu + (1.0 - fi) * (1.0 - uu))
                    * (fj * vv + (1.0 - fj) * (1.0 - vv))
                    * (fk * ww + (1.0 - fk) * (1.0 - ww))
                    * c[i][j][k].dot(weight_v);
            }
        }
    }

    acc
}
