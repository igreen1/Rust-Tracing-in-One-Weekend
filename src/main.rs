#![warn(clippy::pedantic)] // Enable the pedantic lint group as warnings
#![warn(clippy::nursery)] // Enable the nursery lint group as warnings

pub mod math_utils;
pub mod ray_tracing;

use crate::math_utils::point::Point;
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
    let center_sphere = Box::new(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5));
    let ground_sphere = Box::new(Sphere::new(Point::new(0.0, -100.5, -2.0), 100.0));

    Group::new(vec![center_sphere, ground_sphere])
}
