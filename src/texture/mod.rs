pub mod checker;
pub mod image;
pub mod noise;
mod perlin;
pub mod solid_color;

use crate::{color::Color, interval::Clamp, vec3::Point3};

#[derive(Clone)]
#[non_exhaustive]
pub enum Texture {
    SolidColor(solid_color::SolidColor),
    Checker(checker::Checker),
    Image(image::Image),
    Noise(noise::Noise),
    TurbNoise(noise::TurbNoise),
    Marble(noise::MarbleNoise),
}

impl From<solid_color::SolidColor> for Texture {
    fn from(value: solid_color::SolidColor) -> Self {
        Texture::SolidColor(value)
    }
}

impl From<checker::Checker> for Texture {
    fn from(value: checker::Checker) -> Self {
        Texture::Checker(value)
    }
}

impl From<image::Image> for Texture {
    fn from(value: image::Image) -> Self {
        Texture::Image(value)
    }
}

impl From<noise::Noise> for Texture {
    fn from(value: noise::Noise) -> Self {
        Texture::Noise(value)
    }
}

impl From<noise::TurbNoise> for Texture {
    fn from(value: noise::TurbNoise) -> Self {
        Texture::TurbNoise(value)
    }
}

impl From<noise::MarbleNoise> for Texture {
    fn from(value: noise::MarbleNoise) -> Self {
        Texture::Marble(value)
    }
}

impl Texture {
    pub fn value(&self, u: f64, v: f64, p: Point3) -> Color {
        use Texture::*;
        match self {
            SolidColor(c) => c.albedo,
            Checker(c) => {
                let x_int = (c.inv_scale * p.x()).floor() as i32;
                let y_int = (c.inv_scale * p.y()).floor() as i32;
                let z_int = (c.inv_scale * p.z()).floor() as i32;

                let is_even = (x_int + y_int + z_int) % 2 == 0;

                if is_even {
                    c.even.value(u, v, p)
                } else {
                    c.odd.value(u, v, p)
                }
            }
            Image(img) => {
                let (width, height) = img.image.dimensions();

                // if image is empty, return solid cyan for debugging
                if width <= 0 && height <= 0 {
                    return Color::new(0.0, 1.0, 1.0);
                }

                // Clamp input texture coordinates to [0,1] x [1,0]
                let interval = 0.0..=1.0;
                let u = interval.clamp(u);
                let v = 1.0 - interval.clamp(v);

                let i = (u * width as f64) as u32;
                let j = (v * height as f64) as u32;
                let pixel = img.image.get_pixel(i, j).0;

                const COLOR_SCALE: f64 = 1.0 / 255.0;

                let gamma_r = COLOR_SCALE * pixel[0] as f64;
                let gamma_g = COLOR_SCALE * pixel[1] as f64;
                let gamma_b = COLOR_SCALE * pixel[2] as f64;

                Color::new(gamma_r * gamma_r, gamma_g * gamma_g, gamma_b * gamma_b)
            }
            Noise(n) => Color::white() * 0.5 * (1.0 + n.noise.noise(n.scale * p)),
            TurbNoise(n) => Color::white() * n.noise.turb(n.scale * p, n.depth),
            Marble(n) => {
                Color::half_grey()
                    * (1.0 + f64::sin(n.scale * p.z() + 10.0 * n.noise.turb(p, n.depth)))
            }
        }
    }
}
