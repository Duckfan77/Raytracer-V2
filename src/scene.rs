#![allow(dead_code)]

use std::f64::consts::PI;

use rand::{
    distributions::{Distribution, Uniform},
    Rng,
};

use crate::{
    camera::Camera,
    color::Color,
    hittable::{bvh::BvhNode, hittable_list::HittableList, quad::Quad, sphere::Sphere, Hittable},
    material::{dielectric::*, lambertian::Lambertian, metal::Metal},
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
    }
}
