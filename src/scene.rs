#![allow(dead_code)]

use crate::{
    camera::Camera,
    color::Color,
    hittable::{hittable_list::HittableList, sphere::Sphere, Hittable},
    material::{dielectric::*, lambertian::Lambertian, metal::Metal, Mat},
    vec3::Point3,
};

pub fn two_lambertians() -> (Hittable, Camera) {
    let mut world = HittableList::new();

    let mat: Mat = Lambertian::new(Color::new(0.5, 0.5, 0.5)).into();

    world.add(Sphere::new(&Point3::new(0.0, 0.0, -1.0), 0.5, mat.clone()));
    world.add(Sphere::new(
        &Point3::new(0.0, -100.5, -1.0),
        100.0,
        mat.clone(),
    ));

    let cam = Camera {
        aspect_ratio: 16.0 / 9.0,
        image_width: 480,
        samples_per_pixel: 100,
        max_depth: 50,
    };

    (world.into(), cam)
}

pub fn smooth_metal() -> (Hittable, Camera) {
    let mut world = HittableList::new();

    let material_ground: Mat = Lambertian::new(Color::new(0.8, 0.8, 0.0)).into();
    let material_center: Mat = Lambertian::new(Color::new(0.1, 0.2, 0.5)).into();
    let material_left: Mat = Metal::new(Color::new(0.8, 0.8, 0.8), 0.0).into();
    let material_right: Mat = Metal::new(Color::new(0.8, 0.6, 0.2), 0.0).into();

    world.add(Sphere::new(
        &Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    ));
    world.add(Sphere::new(
        &Point3::new(0.0, 0.0, -1.2),
        0.5,
        material_center,
    ));
    world.add(Sphere::new(
        &Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    ));
    world.add(Sphere::new(
        &Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    ));

    let cam = Camera {
        aspect_ratio: 16.0 / 9.0,
        image_width: 480,
        samples_per_pixel: 100,
        max_depth: 50,
    };

    (world.into(), cam)
}

pub fn fuzzed_metal() -> (Hittable, Camera) {
    let mut world = HittableList::new();

    let material_ground: Mat = Lambertian::new(Color::new(0.8, 0.8, 0.0)).into();
    let material_center: Mat = Lambertian::new(Color::new(0.1, 0.2, 0.5)).into();
    let material_left: Mat = Metal::new(Color::new(0.8, 0.8, 0.8), 0.3).into();
    let material_right: Mat = Metal::new(Color::new(0.8, 0.6, 0.2), 1.0).into();

    world.add(Sphere::new(
        &Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    ));
    world.add(Sphere::new(
        &Point3::new(0.0, 0.0, -1.2),
        0.5,
        material_center,
    ));
    world.add(Sphere::new(
        &Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    ));
    world.add(Sphere::new(
        &Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    ));

    let cam = Camera {
        aspect_ratio: 16.0 / 9.0,
        image_width: 480,
        samples_per_pixel: 100,
        max_depth: 50,
    };

    (world.into(), cam)
}

pub fn solid_glass() -> (Hittable, Camera) {
    let mut world = HittableList::new();

    let material_ground: Mat = Lambertian::new(Color::new(0.8, 0.8, 0.0)).into();
    let material_center: Mat = Lambertian::new(Color::new(0.1, 0.2, 0.5)).into();
    let material_left: Mat = Dielectric::new(RI_GLASS).into();
    let material_right: Mat = Metal::new(Color::new(0.8, 0.6, 0.2), 1.0).into();

    world.add(Sphere::new(
        &Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    ));
    world.add(Sphere::new(
        &Point3::new(0.0, 0.0, -1.2),
        0.5,
        material_center,
    ));
    world.add(Sphere::new(
        &Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    ));
    world.add(Sphere::new(
        &Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    ));

    let cam = Camera {
        aspect_ratio: 16.0 / 9.0,
        image_width: 480,
        samples_per_pixel: 100,
        max_depth: 50,
    };

    (world.into(), cam)
}

pub fn air_bubble() -> (Hittable, Camera) {
    let mut world = HittableList::new();

    let material_ground: Mat = Lambertian::new(Color::new(0.8, 0.8, 0.0)).into();
    let material_center: Mat = Lambertian::new(Color::new(0.1, 0.2, 0.5)).into();
    let material_left: Mat = Dielectric::new(RI_AIR / RI_WATER).into();
    let material_right: Mat = Metal::new(Color::new(0.8, 0.6, 0.2), 1.0).into();

    world.add(Sphere::new(
        &Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    ));
    world.add(Sphere::new(
        &Point3::new(0.0, 0.0, -1.2),
        0.5,
        material_center,
    ));
    world.add(Sphere::new(
        &Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    ));
    world.add(Sphere::new(
        &Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    ));

    let cam = Camera {
        aspect_ratio: 16.0 / 9.0,
        image_width: 480,
        samples_per_pixel: 100,
        max_depth: 50,
    };

    (world.into(), cam)
}

pub fn hollow_glass() -> (Hittable, Camera) {
    let mut world = HittableList::new();

    let material_ground: Mat = Lambertian::new(Color::new(0.8, 0.8, 0.0)).into();
    let material_center: Mat = Lambertian::new(Color::new(0.1, 0.2, 0.5)).into();
    let material_left: Mat = Dielectric::new(RI_GLASS).into();
    let material_bubble: Mat = Dielectric::new(RI_AIR / RI_GLASS).into();
    let material_right: Mat = Metal::new(Color::new(0.8, 0.6, 0.2), 1.0).into();

    world.add(Sphere::new(
        &Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    ));
    world.add(Sphere::new(
        &Point3::new(0.0, 0.0, -1.2),
        0.5,
        material_center,
    ));
    world.add(Sphere::new(
        &Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    ));
    world.add(Sphere::new(
        &Point3::new(-1.0, 0.0, -1.0),
        0.4,
        material_bubble,
    ));
    world.add(Sphere::new(
        &Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    ));

    let cam = Camera {
        aspect_ratio: 16.0 / 9.0,
        image_width: 480,
        samples_per_pixel: 100,
        max_depth: 50,
    };

    (world.into(), cam)
}
