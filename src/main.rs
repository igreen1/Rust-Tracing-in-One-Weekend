#![warn(clippy::pedantic)] // Enable the pedantic lint group as warnings
#![warn(clippy::nursery)] // Enable the nursery lint group as warnings

pub mod math_utils;
pub mod ray_tracing;

use std::rc::Rc;

use crate::math_utils::point::Point;
use crate::ray_tracing::color::Color;
use crate::ray_tracing::materials::{
    dielectric::DielectricMaterial, lambertian::LambertianMaterial, metal::MetalMaterial,
};
use crate::ray_tracing::{
    camera::camera::Camera,
    shapes::{group::Group, sphere::Sphere},
};

fn main() {
    // get the objects we're rendering in the world
    let world = make_world();

    let camera = Camera::default();

    camera.render(world);
}

fn make_world() -> Group {
    // green ground
    let material_ground = LambertianMaterial::new(Color::new(0.8, 0.8, 0.0).unwrap());
    let material_center = LambertianMaterial::new(Color::new(0.1, 0.2, 0.5).unwrap());
    let material_left = DielectricMaterial::new(1.5);
    // let material_left = MetalMaterial::new(Color::new(0.8, 0.8, 0.8).unwrap());
    let material_right = MetalMaterial::new(Color::new(0.8, 0.6, 0.2).unwrap());

    let center_sphere = Box::new(Sphere::new(
        Point::new(0.0, 0.0, -1.2),
        0.5,
        Rc::new(material_center),
    ));
    let ground_sphere = Box::new(Sphere::new(
        Point::new(0.0, -100.5, -2.0),
        100.0,
        Rc::new(material_ground),
    ));
    let left_sphere = Box::new(Sphere::new(
        Point::new(-1.0, 0.0, -1.0),
        0.5,
        Rc::new(material_left),
    ));
    let right_sphere = Box::new(Sphere::new(
        Point::new(1.0, 0.0, -1.0),
        0.5,
        Rc::new(material_right),
    ));

    Group::new(vec![
        center_sphere,
        ground_sphere,
        right_sphere,
        left_sphere,
    ])
}
