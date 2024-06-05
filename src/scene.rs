#![allow(dead_code)]

use std::f64::consts::PI;

use rand::{
    distributions::{Distribution, Uniform},
    Rng,
};

use crate::{
    camera::Camera,
    color::Color,
    hittable::{
        bvh::BvhNode,
        constant_medium::ConstantMedium,
        hittable_list::HittableList,
        instance::{Translate, YRotate},
        quad::Quad,
        sphere::Sphere,
        Hittable,
    },
    material::{dielectric::*, emissive::DiffuseLight, lambertian::Lambertian, metal::Metal},
    texture::{
        checker::Checker,
        image::Image,
        noise::{MarbleNoise, Noise, TurbNoise},
    },
    vec3::{Point3, Vec3},
};

// Worlds

pub fn two_lambertians() -> Hittable {
    let mut world = HittableList::new();

    let mat = Lambertian::new(Color::half_grey());

    world.add(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, mat.clone()));
    world.add(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, mat));

    world.into()
}

pub fn smooth_metal() -> Hittable {
    let mut world = HittableList::new();

    let material_ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(Color::new(0.1, 0.2, 0.5));
    let material_left = Metal::new(Color::new(0.8, 0.8, 0.8), 0.0);
    let material_right = Metal::new(Color::new(0.8, 0.6, 0.2), 0.0);

    world.add(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    ));
    world.add(Sphere::new(
        Point3::new(0.0, 0.0, -1.2),
        0.5,
        material_center,
    ));
    world.add(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    ));
    world.add(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    ));

    world.into()
}

pub fn fuzzed_metal() -> Hittable {
    let mut world = HittableList::new();

    let material_ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(Color::new(0.1, 0.2, 0.5));
    let material_left = Metal::new(Color::new(0.8, 0.8, 0.8), 0.3);
    let material_right = Metal::new(Color::new(0.8, 0.6, 0.2), 1.0);

    world.add(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    ));
    world.add(Sphere::new(
        Point3::new(0.0, 0.0, -1.2),
        0.5,
        material_center,
    ));
    world.add(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    ));
    world.add(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    ));

    world.into()
}

pub fn solid_glass() -> Hittable {
    let mut world = HittableList::new();

    let material_ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(Color::new(0.1, 0.2, 0.5));
    let material_left = Dielectric::new(RI_GLASS);
    let material_right = Metal::new(Color::new(0.8, 0.6, 0.2), 1.0);

    world.add(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    ));
    world.add(Sphere::new(
        Point3::new(0.0, 0.0, -1.2),
        0.5,
        material_center,
    ));
    world.add(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    ));
    world.add(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    ));

    world.into()
}

pub fn air_bubble() -> Hittable {
    let mut world = HittableList::new();

    let material_ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(Color::new(0.1, 0.2, 0.5));
    let material_left = Dielectric::new(RI_AIR / RI_WATER);
    let material_right = Metal::new(Color::new(0.8, 0.6, 0.2), 1.0);

    world.add(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    ));
    world.add(Sphere::new(
        Point3::new(0.0, 0.0, -1.2),
        0.5,
        material_center,
    ));
    world.add(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    ));
    world.add(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    ));

    world.into()
}

pub fn hollow_glass() -> Hittable {
    let mut world = HittableList::new();

    let material_ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(Color::new(0.1, 0.2, 0.5));
    let material_left = Dielectric::new(RI_GLASS);
    let material_bubble = Dielectric::new(RI_AIR / RI_GLASS);
    let material_right = Metal::new(Color::new(0.8, 0.6, 0.2), 1.0);

    world.add(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    ));
    world.add(Sphere::new(
        Point3::new(0.0, 0.0, -1.2),
        0.5,
        material_center,
    ));
    world.add(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    ));
    world.add(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.4,
        material_bubble,
    ));
    world.add(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    ));

    world.into()
}

pub fn two_spheres() -> Hittable {
    let mut world = HittableList::new();

    let r = (PI / 4.0).cos();
    let material_left = Lambertian::new(Color::new(0.0, 0.0, 1.0));
    let material_right = Lambertian::new(Color::new(1.0, 0.0, 0.0));

    world.add(Sphere::new(Point3::new(-r, 0.0, -1.0), r, material_left));
    world.add(Sphere::new(Point3::new(r, 0.0, -1.0), r, material_right));

    world.into()
}

pub fn random_spheres() -> Hittable {
    let mut world = HittableList::new();

    let ground_material = Lambertian::new(Color::half_grey());
    world.add(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    ));

    const BALL_RADIUS: f64 = 0.2;

    // Useful values used throughout the loops
    let mut rng = rand::thread_rng();
    let ball_dist_center = Point3::new(4.0, BALL_RADIUS, 0.0);
    let metal_color_dist = Uniform::from(0.5..1.0);
    let metal_fuzz_dist = Uniform::from(0.0..0.5);
    let diffuse_color_dist = Uniform::from(0.0..1.0);

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = rng.gen();
            let center = Point3::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                BALL_RADIUS,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );

            // Clip off any outside the disk of size 0.9 centered at ball_dist_center
            if (center - ball_dist_center).length() > 0.9 {
                match choose_mat {
                    ..=0.8 => {
                        // Diffuse
                        let albedo = Color::random_range(diffuse_color_dist, &mut rng)
                            * Color::random_range(diffuse_color_dist, &mut rng);
                        let mat = Lambertian::new(albedo);
                        world.add(Sphere::new(center, BALL_RADIUS, mat));
                    }
                    ..=0.95 => {
                        // metal
                        let albedo = Color::random_range(metal_color_dist, &mut rng);
                        let fuzz = rng.sample(metal_fuzz_dist);
                        let mat = Metal::new(albedo, fuzz);
                        world.add(Sphere::new(center, BALL_RADIUS, mat));
                    }
                    _ => {
                        // Glass
                        let mat = Dielectric::new(RI_GLASS);
                        world.add(Sphere::new(center, BALL_RADIUS, mat));
                    }
                }
            }
        }
    }

    let mat1 = Dielectric::new(RI_GLASS);
    world.add(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, mat1));

    let mat2 = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    world.add(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, mat2));

    let mat3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
    world.add(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, mat3));

    world.into()
}

pub fn bouncing_random_spheres() -> Hittable {
    let mut world = HittableList::new();

    let ground_material = Lambertian::new(Color::half_grey());
    world.add(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    ));

    const BALL_RADIUS: f64 = 0.2;

    // Useful values used throughout the loops
    let mut rng = rand::thread_rng();
    let ball_dist_center = Point3::new(4.0, BALL_RADIUS, 0.0);
    let metal_color_dist = Uniform::from(0.5..1.0);
    let metal_fuzz_dist = Uniform::from(0.0..0.5);
    let diffuse_color_dist = Uniform::from(0.0..1.0);
    let diffuse_move_dist = Uniform::from(0.0..0.5);

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = rng.gen();
            let center = Point3::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                BALL_RADIUS,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );

            // Clip off any outside the disk of size 0.9 centered at ball_dist_center
            if (center - ball_dist_center).length() > 0.9 {
                match choose_mat {
                    ..=0.8 => {
                        // Diffuse
                        let albedo = Color::random_range(diffuse_color_dist, &mut rng)
                            * Color::random_range(diffuse_color_dist, &mut rng);
                        let mat = Lambertian::new(albedo);
                        let center1 =
                            center + Vec3::new(0.0, diffuse_move_dist.sample(&mut rng), 0.0);
                        world.add(Sphere::new_moving(center, center1, BALL_RADIUS, mat));
                    }
                    ..=0.95 => {
                        // metal
                        let albedo = Color::random_range(metal_color_dist, &mut rng);
                        let fuzz = rng.sample(metal_fuzz_dist);
                        let mat = Metal::new(albedo, fuzz);
                        world.add(Sphere::new(center, BALL_RADIUS, mat));
                    }
                    _ => {
                        // Glass
                        let mat = Dielectric::new(RI_GLASS);
                        world.add(Sphere::new(center, BALL_RADIUS, mat));
                    }
                }
            }
        }
    }

    let mat1 = Dielectric::new(RI_GLASS);
    world.add(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, mat1));

    let mat2 = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    world.add(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, mat2));

    let mat3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
    world.add(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, mat3));

    let world = BvhNode::from_list(world);

    world.into()
}

pub fn bouncing_random_spheres_checkerboard() -> Hittable {
    let mut world = HittableList::new();

    let checker = Checker::from_colors(0.32, Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9));
    let ground_material = Lambertian::from_texture(checker);
    world.add(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    ));

    const BALL_RADIUS: f64 = 0.2;

    // Useful values used throughout the loops
    let mut rng = rand::thread_rng();
    let ball_dist_center = Point3::new(4.0, BALL_RADIUS, 0.0);
    let metal_color_dist = Uniform::from(0.5..1.0);
    let metal_fuzz_dist = Uniform::from(0.0..0.5);
    let diffuse_color_dist = Uniform::from(0.0..1.0);
    let diffuse_move_dist = Uniform::from(0.0..0.5);

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = rng.gen();
            let center = Point3::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                BALL_RADIUS,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );

            // Clip off any outside the disk of size 0.9 centered at ball_dist_center
            if (center - ball_dist_center).length() > 0.9 {
                match choose_mat {
                    ..=0.8 => {
                        // Diffuse
                        let albedo = Color::random_range(diffuse_color_dist, &mut rng)
                            * Color::random_range(diffuse_color_dist, &mut rng);
                        let mat = Lambertian::new(albedo);
                        let center1 =
                            center + Vec3::new(0.0, diffuse_move_dist.sample(&mut rng), 0.0);
                        world.add(Sphere::new_moving(center, center1, BALL_RADIUS, mat));
                    }
                    ..=0.95 => {
                        // metal
                        let albedo = Color::random_range(metal_color_dist, &mut rng);
                        let fuzz = rng.sample(metal_fuzz_dist);
                        let mat = Metal::new(albedo, fuzz);
                        world.add(Sphere::new(center, BALL_RADIUS, mat));
                    }
                    _ => {
                        // Glass
                        let mat = Dielectric::new(RI_GLASS);
                        world.add(Sphere::new(center, BALL_RADIUS, mat));
                    }
                }
            }
        }
    }

    let mat1 = Dielectric::new(RI_GLASS);
    world.add(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, mat1));

    let mat2 = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    world.add(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, mat2));

    let mat3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
    world.add(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, mat3));

    let world = BvhNode::from_list(world);

    world.into()
}

pub fn two_checkered_spheres() -> Hittable {
    let mut world = HittableList::new();

    let checker = Checker::from_colors(0.32, Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9));

    world.add(Sphere::new(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        Lambertian::from_texture(checker.clone()),
    ));
    world.add(Sphere::new(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        Lambertian::from_texture(checker.clone()),
    ));

    world.into()
}

pub fn earth() -> Hittable {
    let earth_texture = Image::new("src/assets/earthmap.jpg");
    let earth_surface = Lambertian::from_texture(earth_texture);
    let globe = Sphere::new(Point3::new(0.0, 0.0, 0.0), 2.0, earth_surface);

    globe.into()
}

pub fn perlin_spheres() -> Hittable {
    let mut world = HittableList::new();

    let per_text = Noise::new(4.0);
    let per_mat = Lambertian::from_texture(per_text);
    world.add(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        per_mat.clone(),
    ));
    world.add(Sphere::new(Point3::new(0.0, 2.0, 0.0), 2.0, per_mat));

    world.into()
}

pub fn turbulent_spheres() -> Hittable {
    let mut world = HittableList::new();

    let per_text = TurbNoise::new(1.0, 7);
    let per_mat = Lambertian::from_texture(per_text);
    world.add(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        per_mat.clone(),
    ));
    world.add(Sphere::new(Point3::new(0.0, 2.0, 0.0), 2.0, per_mat));

    world.into()
}

pub fn marble_spheres() -> Hittable {
    let mut world = HittableList::new();

    let per_text = MarbleNoise::new(4.0, 7);
    let per_mat = Lambertian::from_texture(per_text);
    world.add(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        per_mat.clone(),
    ));
    world.add(Sphere::new(Point3::new(0.0, 2.0, 0.0), 2.0, per_mat));

    world.into()
}

pub fn quads() -> Hittable {
    let mut world = HittableList::new();

    // Materials
    let left_red = Lambertian::new(Color::new(1.0, 0.2, 0.2));
    let back_green = Lambertian::new(Color::new(0.2, 1.0, 0.2));
    let right_blue = Lambertian::new(Color::new(0.2, 0.2, 1.0));
    let top_orange = Lambertian::new(Color::new(1.0, 0.5, 0.0));
    let bottom_teal = Lambertian::new(Color::new(0.2, 0.8, 0.8));

    // Quads
    world.add(Quad::new(
        Point3::new(-3.0, -2.0, 5.0),
        Vec3::new(0.0, 0.0, -4.0),
        Vec3::new(0.0, 4.0, 0.0),
        left_red,
    ));
    world.add(Quad::new(
        Point3::new(-2.0, -2.0, 0.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 4.0, 0.0),
        back_green,
    ));
    world.add(Quad::new(
        Point3::new(3.0, -2.0, 1.0),
        Vec3::new(0.0, 0.0, 4.0),
        Vec3::new(0.0, 4.0, 0.0),
        right_blue,
    ));
    world.add(Quad::new(
        Point3::new(-2.0, 3.0, 1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 4.0),
        top_orange,
    ));
    world.add(Quad::new(
        Point3::new(-2.0, -3.0, 5.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -4.0),
        bottom_teal,
    ));

    world.into()
}

pub fn symbol() -> Hittable {
    let mut world = HittableList::new();

    let per_text = TurbNoise::new(1.0, 7);
    world.add(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Metal::new(Color::new(1.0, 0.0, 0.0), 0.0),
    ));
    world.add(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Lambertian::from_texture(per_text),
    ));

    let diff_light = DiffuseLight::new(Color::new(0.0, 2.0, 8.0));
    world.add(Quad::new(
        Point3::new(3.0, 10.0, 3.0),
        Vec3::new(-6.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -6.0),
        diff_light,
    ));

    let sphere_light = DiffuseLight::new(Color::new(4.0, 4.0, 4.0));
    world.add(Sphere::new(
        Point3::new(-5.0, 2.0, -15.0 / 13.0),
        2.5,
        sphere_light,
    ));

    world.into()
}

pub fn cornell_box() -> Hittable {
    let mut world = HittableList::new();

    let red = Lambertian::new(Color::new(0.65, 0.05, 0.05));
    let white = Lambertian::new(Color::new(0.73, 0.73, 0.73));
    let green = Lambertian::new(Color::new(0.12, 0.45, 0.15));
    let light = DiffuseLight::new(Color::new(15.0, 15.0, 15.0));

    world.add(Quad::new(
        Point3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green,
    ));
    world.add(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red,
    ));
    world.add(Quad::new(
        Point3::new(343.0, 554.0, 332.0),
        Vec3::new(-130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -105.0),
        light,
    ));
    world.add(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    ));
    world.add(Quad::new(
        Point3::new(555.0, 555.0, 555.0),
        Vec3::new(-555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -555.0),
        white.clone(),
    ));
    world.add(Quad::new(
        Point3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        white.clone(),
    ));

    let box1 = Quad::new_box(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 330.0, 165.0),
        white.clone(),
    );
    let box1 = YRotate::new(box1, 15.0);
    let box1 = Translate::new(box1, Vec3::new(265.0, 0.0, 295.0));
    world.add(box1);

    let box2 = Quad::new_box(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 165.0, 165.0),
        white,
    );
    let box2 = YRotate::new(box2, -18.0);
    let box2 = Translate::new(box2, Vec3::new(130.0, 0.0, 65.0));
    world.add(box2);

    world.into()
}

pub fn cornell_smoke() -> Hittable {
    let mut world = HittableList::new();

    let red = Lambertian::new(Color::new(0.65, 0.05, 0.05));
    let white = Lambertian::new(Color::new(0.73, 0.73, 0.73));
    let green = Lambertian::new(Color::new(0.12, 0.45, 0.15));
    let light = DiffuseLight::new(Color::new(15.0, 15.0, 15.0));

    world.add(Quad::new(
        Point3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green,
    ));
    world.add(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red,
    ));
    world.add(Quad::new(
        Point3::new(113.0, 554.0, 127.0),
        Vec3::new(330.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 305.0),
        light,
    ));
    world.add(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    ));
    world.add(Quad::new(
        Point3::new(555.0, 555.0, 555.0),
        Vec3::new(-555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -555.0),
        white.clone(),
    ));
    world.add(Quad::new(
        Point3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        white.clone(),
    ));

    let box1 = Quad::new_box(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 330.0, 165.0),
        white.clone(),
    );
    let box1 = YRotate::new(box1, 15.0);
    let box1 = Translate::new(box1, Vec3::new(265.0, 0.0, 295.0));
    world.add(ConstantMedium::new(box1, 0.01, Color::black()));

    let box2 = Quad::new_box(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 165.0, 165.0),
        white,
    );
    let box2 = YRotate::new(box2, -18.0);
    let box2 = Translate::new(box2, Vec3::new(130.0, 0.0, 65.0));
    world.add(ConstantMedium::new(box2, 0.01, Color::white()));

    world.into()
}

pub fn book2_final() -> Hittable {
    let mut boxes1 = HittableList::new();
    let ground = Lambertian::new(Color::new(0.48, 0.83, 0.53));

    const BOXES_PER_SIDE: usize = 20;
    let height_dist = Uniform::from(1.0..=101.0);
    let mut rng = rand::thread_rng();
    for i in 0..BOXES_PER_SIDE {
        for j in 0..BOXES_PER_SIDE {
            const W: f64 = 100.0;
            let x0 = -1000.0 + i as f64 * W;
            let z0 = -1000.0 + j as f64 * W;
            let y0 = 0.0;
            let x1 = x0 + W;
            let z1 = z0 + W;
            let y1 = height_dist.sample(&mut rng);

            boxes1.add(Quad::new_box(
                Point3::new(x0, y0, z0),
                Point3::new(x1, y1, z1),
                ground.clone(),
            ))
        }
    }

    let mut world = HittableList::new();

    world.add(BvhNode::from_list(boxes1));

    let light = DiffuseLight::new(7.0 * Color::white());
    world.add(Quad::new(
        Point3::new(123.0, 554.0, 147.0),
        Vec3::new(300.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 256.0),
        light,
    ));

    let center0 = Point3::new(400.0, 400.0, 200.0);
    let center1 = center0 + Vec3::new(30.0, 0.0, 0.0);
    let sphere_material = Lambertian::new(Color::new(0.7, 0.3, 0.1));
    world.add(Sphere::new_moving(center0, center1, 50.0, sphere_material));

    world.add(Sphere::new(
        Point3::new(260.0, 150.0, 45.0),
        50.0,
        Dielectric::new(RI_GLASS),
    ));
    world.add(Sphere::new(
        Point3::new(0.0, 150.0, 145.0),
        50.0,
        Metal::new(Color::new(0.8, 0.8, 0.9), 1.0),
    ));

    let boundary = Sphere::new(
        Point3::new(360.0, 150.0, 145.0),
        70.0,
        Dielectric::new(RI_GLASS),
    );
    world.add(boundary.clone());
    world.add(ConstantMedium::new(
        boundary,
        0.2,
        Color::new(0.2, 0.4, 0.9),
    ));
    let boundary = Sphere::new(
        Point3::new(0.0, 0.0, 0.0),
        5000.0,
        Dielectric::new(RI_GLASS),
    );
    world.add(ConstantMedium::new(boundary, 0.0001, Color::white()));

    let earth_mat = Lambertian::from_texture(Image::new("src/assets/earthmap.jpg"));
    world.add(Sphere::new(
        Point3::new(400.0, 200.0, 400.0),
        100.0,
        earth_mat,
    ));
    let per_tex = MarbleNoise::new(0.2, 7);
    world.add(Sphere::new(
        Point3::new(220.0, 280.0, 300.0),
        80.0,
        Lambertian::from_texture(per_tex),
    ));

    let mut spheres = HittableList::new();
    let white = Lambertian::new(0.73 * Color::white());
    const SPHERE_COUNT: usize = 1000;
    let sphere_dist = Uniform::from(0.0..165.0);
    for _ in 0..SPHERE_COUNT {
        spheres.add(Sphere::new(
            Point3::random_dist(&sphere_dist, &mut rng),
            10.0,
            white.clone(),
        ))
    }

    world.add(Translate::new(
        YRotate::new(BvhNode::from_list(spheres), 15.0),
        Vec3::new(-100.0, 270.0, 395.0),
    ));

    world.into()
}

// Camera positions and layouts

pub fn unmoved_camera() -> Camera {
    Camera {
        aspect_ratio: 16.0 / 9.0,
        image_width: 480,
        samples_per_pixel: 100,
        max_depth: 50,
        vfov: 90.0,

        look_from: Vec3::new(0.0, 0.0, 0.0),
        look_at: Vec3::new(0.0, 0.0, -1.0),
        v_up: Vec3::new(0.0, 1.0, 0.0),

        defocus_angle: 0.0,
        focus_dist: 1.0,

        background: None,
    }
}

pub fn far_camera() -> Camera {
    Camera {
        aspect_ratio: 16.0 / 9.0,
        image_width: 480,
        samples_per_pixel: 100,
        max_depth: 50,
        vfov: 90.0,

        look_from: Vec3::new(-2.0, 2.0, 1.0),
        look_at: Vec3::new(0.0, 0.0, -1.0),
        v_up: Vec3::new(0.0, 1.0, 0.0),

        defocus_angle: 0.0,
        focus_dist: 2.0 * (3_f64).sqrt(),

        background: None,
    }
}

pub fn far_camera_zoomed() -> Camera {
    Camera {
        aspect_ratio: 16.0 / 9.0,
        image_width: 480,
        samples_per_pixel: 100,
        max_depth: 50,
        vfov: 20.0,

        look_from: Vec3::new(-2.0, 2.0, 1.0),
        look_at: Vec3::new(0.0, 0.0, -1.0),
        v_up: Vec3::new(0.0, 1.0, 0.0),

        defocus_angle: 0.0,
        focus_dist: 2.0 * (3_f64).sqrt(),

        background: None,
    }
}

pub fn far_camera_zoomed_large_aperture() -> Camera {
    Camera {
        aspect_ratio: 16.0 / 9.0,
        image_width: 480,
        samples_per_pixel: 100,
        max_depth: 50,
        vfov: 20.0,

        look_from: Vec3::new(-2.0, 2.0, 1.0),
        look_at: Vec3::new(0.0, 0.0, -1.0),
        v_up: Vec3::new(0.0, 1.0, 0.0),

        defocus_angle: 10.0,
        focus_dist: 3.4,

        background: None,
    }
}

pub fn random_spheres_camera() -> Camera {
    Camera {
        aspect_ratio: 16.0 / 9.0,
        image_width: 1200,
        samples_per_pixel: 500,
        max_depth: 50,

        vfov: 20.0,
        look_from: Point3::new(13.0, 2.0, 3.0),
        look_at: Point3::new(0.0, 0.0, 0.0),
        v_up: Vec3::new(0.0, 1.0, 0.0),

        defocus_angle: 0.6,
        focus_dist: 10.0,

        background: None,
    }
}

pub fn random_spheres_camera_fast() -> Camera {
    Camera {
        aspect_ratio: 16.0 / 9.0,
        image_width: 400,
        samples_per_pixel: 100,
        max_depth: 50,

        vfov: 20.0,
        look_from: Point3::new(13.0, 2.0, 3.0),
        look_at: Point3::new(0.0, 0.0, 0.0),
        v_up: Vec3::new(0.0, 1.0, 0.0),

        defocus_angle: 0.6,
        focus_dist: 10.0,

        background: None,
    }
}

pub fn two_spheres_camera() -> Camera {
    Camera {
        aspect_ratio: 16.0 / 9.0,
        image_width: 480,
        samples_per_pixel: 100,
        max_depth: 50,

        vfov: 20.0,
        look_from: Point3::new(13.0, 2.0, 3.0),
        look_at: Point3::new(0.0, 0.0, 0.0),
        v_up: Point3::new(0.0, 1.0, 0.0),

        defocus_angle: 0.0,
        focus_dist: 10.0,

        background: None,
    }
}

pub fn earth_camera() -> Camera {
    Camera {
        aspect_ratio: 16.0 / 9.0,
        image_width: 480,
        samples_per_pixel: 100,
        max_depth: 50,

        vfov: 20.0,
        look_from: Point3::new(0.0, 0.0, 12.0),
        look_at: Point3::new(0.0, 0.0, 0.0),
        v_up: Point3::new(0.0, 1.0, 0.0),

        defocus_angle: 0.0,
        focus_dist: 10.0,

        background: None,
    }
}

pub fn quads_camera() -> Camera {
    Camera {
        aspect_ratio: 1.0,
        image_width: 480,
        samples_per_pixel: 100,
        max_depth: 50,

        vfov: 80.0,
        look_from: Point3::new(0.0, 0.0, 9.0),
        look_at: Point3::new(0.0, 0.0, 0.0),
        v_up: Vec3::new(0.0, 1.0, 0.0),

        defocus_angle: 0.0,
        focus_dist: 10.0,

        background: None,
    }
}

pub fn symbol_camera() -> Camera {
    Camera {
        aspect_ratio: 16.0 / 9.0,
        image_width: 3840,
        //image_width: 1280,
        samples_per_pixel: 500,
        max_depth: 50,
        background: Some(Color::black()),

        vfov: 20.0,
        look_from: Point3::new(26.0, 3.0, 6.0),
        look_at: Point3::new(0.0, 2.0, 0.0),
        v_up: Vec3::new(0.0, 1.0, 0.0),

        defocus_angle: 0.0,
        focus_dist: 10.0,
    }
}

pub fn cornell_box_cam() -> Camera {
    Camera {
        aspect_ratio: 1.0,
        image_width: 600,
        samples_per_pixel: 200,
        max_depth: 50,
        background: Some(Color::black()),

        vfov: 40.0,
        look_from: Point3::new(278.0, 278.0, -800.0),
        look_at: Point3::new(278.0, 278.0, 0.0),
        v_up: Vec3::new(0.0, 1.0, 0.0),

        defocus_angle: 0.0,
        focus_dist: 10.0,
    }
}

pub fn book2_final_camera(image_width: u32, samples_per_pixel: u32, max_depth: u32) -> Camera {
    Camera {
        aspect_ratio: 1.0,
        image_width,
        samples_per_pixel,
        max_depth,
        background: Some(Color::black()),

        vfov: 40.0,
        look_from: Point3::new(478.0, 278.0, -600.0),
        look_at: Point3::new(278.0, 278.0, 0.0),
        v_up: Vec3::new(0.0, 1.0, 0.0),

        defocus_angle: 0.0,
        focus_dist: 10.0,
    }
}
