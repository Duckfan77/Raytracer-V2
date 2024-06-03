#![allow(dead_code)]

use std::f64::consts::PI;

use crate::{
    camera::Camera,
    color::Color,
    hittable::{hittable_list::HittableList, sphere::Sphere, Hittable},
    material::{dielectric::*, lambertian::Lambertian, metal::Metal, Mat},
    vec3::{Point3, Vec3},
};

// Worlds

pub fn two_lambertians() -> Hittable {
    let mut world = HittableList::new();

    let mat: Mat = Lambertian::new(Color::new(0.5, 0.5, 0.5)).into();

    world.add(Sphere::new(&Point3::new(0.0, 0.0, -1.0), 0.5, mat.clone()));
    world.add(Sphere::new(
        &Point3::new(0.0, -100.5, -1.0),
        100.0,
        mat.clone(),
    ));

    world.into()
}

pub fn smooth_metal() -> Hittable {
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

    world.into()
}

pub fn fuzzed_metal() -> Hittable {
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

    world.into()
}

pub fn solid_glass() -> Hittable {
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

    world.into()
}

pub fn air_bubble() -> Hittable {
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

    world.into()
}

pub fn hollow_glass() -> Hittable {
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

    world.into()
}

pub fn two_spheres() -> Hittable {
    let mut world = HittableList::new();

    let r = (PI / 4.0).cos();
    let material_left: Mat = Lambertian::new(Color::new(0.0, 0.0, 1.0)).into();
    let material_right: Mat = Lambertian::new(Color::new(1.0, 0.0, 0.0)).into();

    world.add(Sphere::new(&Point3::new(-r, 0.0, -1.0), r, material_left));
    world.add(Sphere::new(&Point3::new(r, 0.0, -1.0), r, material_right));

    world.into()
}

// Camera positions and layouts

pub fn default_camera() -> Camera {
    Camera {
        aspect_ratio: 16.0 / 9.0,
        image_width: 480,
        samples_per_pixel: 100,
        max_depth: 50,
        vfov: 90.0,

        look_from: Vec3::new(0.0, 0.0, 0.0),
        look_at: Vec3::new(0.0, 0.0, -1.0),
        v_up: Vec3::new(0.0, 1.0, 0.0),
    }
}
