#![warn(clippy::pedantic)] // Enable the pedantic lint group as warnings
#![warn(clippy::nursery)] // Enable the nursery lint group as warnings

pub mod math_utils;
pub mod ray_tracing;

use std::sync::Arc;

use crate::math_utils::point::Point;
use crate::ray_tracing::color::Color;
use crate::ray_tracing::materials::dielectric::DielectricMaterial;
use crate::ray_tracing::materials::lambertian::LambertianMaterial;
use crate::ray_tracing::materials::material::Scatterer;
use crate::ray_tracing::materials::metal::MetalMaterial;
use crate::ray_tracing::shapes::hittable::Hittable;
use crate::ray_tracing::{
    camera::camera::CameraBuilder,
    shapes::{group::Group, sphere::Sphere},
};

fn main() {
    // get the objects we're rendering in the world
    let world = make_world();

    let camera = CameraBuilder::default()
        .set_look_from(-2.0, 2.0,1.0)
        .make_camera();

    camera.render(world);
}

fn make_world() -> Group {
    // green ground

    // let material_left = LambertianMaterial::new(Color::new(0.0, 0.0, 1.0).unwrap());
    // let material_left = Arc::new(material_left);
    // let material_right = LambertianMaterial::new(Color::new(1.0, 0.0, 0.0).unwrap());
    // let material_right = Arc::new(material_right);

    // let radius = (std::f64::consts::PI/4.0).cos();
    // let left_sphere = Box::new(Sphere::new(
    //     Point::new(-radius, 0.0, -1.0), radius, material_left
    // ));
    // let right_sphere = Box::new(Sphere::new(
    //     Point::new(radius, 0.0, -1.0), radius, material_right
    // ));

    let material_ground = LambertianMaterial::new(Color::new(0.8, 0.8, 0.0).unwrap());
    let material_ground = Arc::new(material_ground);

    let material_center = LambertianMaterial::new(Color::new(0.1, 0.2, 0.5).unwrap());
    let material_center = Arc::new(material_center);

    let material_left = DielectricMaterial::new(1.50);
    let material_left = Arc::new(material_left);

    let material_bubble = DielectricMaterial::new(1.00 / 1.50);
    let material_bubble = Arc::new(material_bubble);

    let material_right = MetalMaterial::new(Color::new(0.8, 0.6, 0.2).unwrap());
    let material_right = Arc::new(material_right);

    let world_elements: Vec<(f64, f64, f64, f64, Arc<dyn Scatterer + Send + Sync>)> = vec![
        (0.0, -100.5, -1.0, 100.0, material_ground),
        (0.0, 0.0, -1.2, 0.5, material_center),
        (-1.0, 0.0, -1.0, 0.5, material_left),
        (-1.0, 0.0, -1.0, 0.4, material_bubble),
        (1.0, 0.0, -1.0, 0.5, material_right),
    ];

    let world_elements = world_elements
        .into_iter()
        .map(|(px, py, pz, radius, material)| {
            Box::new(Sphere::new(Point::new(px, py, pz), radius, material))
                as Box<dyn Hittable + Send + Sync>
        });

    Group::new(world_elements.collect())
}
