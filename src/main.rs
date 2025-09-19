use crate::math_utils::point::Point;
use crate::math_utils::ray::Ray;
use crate::math_utils::vector::Vec3;
use crate::ray_tracing::{
    color::Color,
    shapes::{
        group::Group,
        hittable::{Hittable},
        sphere::Sphere,
    },
};
use indicatif::ProgressIterator;
use core::f64;
use std::io::Write;
pub mod math_utils;
pub mod ray_tracing;

fn main() {
    // *** Start Setup ***
    let ideal_aspect_ratio = 16.0 / 9.0;
    let image_width: isize = 256;
    let image_height = (image_width as f64 / ideal_aspect_ratio).round() as isize;
    // clamp height to 1 at a minimum
    let image_height = if image_height < 1 { 1 } else { image_height };

    // create the viewport
    let image_aspect_ratio = image_width as f64 / image_height as f64;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * image_aspect_ratio;

    // create camera constants
    let camera_center = Point::new(0.0, 0.0, 0.0);
    let focal_length = 1.0;

    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    let pixel_du = viewport_u / (image_width as f64);
    let pixel_dv = viewport_v / (image_height as f64);

    let viewport_upper_left =
        camera_center - Vec3::new(0., 0., focal_length) - viewport_u / 2. - viewport_v / 2.;
    let pixel_00_location = viewport_upper_left + 0.5 * (pixel_du + pixel_dv);
    // *** Start Rendering ***

    // get the objects we're rendering in the world
    let world = make_world();

    // File to write
    let mut file_handle = std::fs::File::create("./image.ppm").unwrap();

    let header = format!("P3\n{} {}\n255\n", image_width, image_height).into_bytes();
    file_handle.write_all(&header).unwrap();

    for j in (0..image_height).progress() {
        for i in 0..image_width {
            let row = j as f64;
            let col = i as f64;

            let pixel_center = pixel_00_location + (col * pixel_du) + (row * pixel_dv);
            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(camera_center, ray_direction);

            let color = get_ray_color(ray, &world);
            write(&mut file_handle, color);
        }
    }
}

fn make_world() -> Group {
    let center_sphere = Box::new(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5));
    let ground_sphere = Box::new(
        Sphere::new(
            Point::new(0.0, -100.5, -2.0),
            100.0
        )
    );

    Group::new(vec![center_sphere, ground_sphere])
}

fn get_ray_color<T>(ray: Ray<f64>, world: &T) -> Color
where
    T: Hittable,
{
    match world.hit(&ray, 0.0, f64::INFINITY) {
        Some(hit_record) => {
            let color_space_vector = 0.5 * (hit_record.normal + Vec3::new(1., 1., 1.));
            Color::new(
                color_space_vector.x,
                color_space_vector.y,
                color_space_vector.z,
            )
            .unwrap()
        }
        None => {
            // default blue to white fade
            let unit_direction = (*ray.get_direction()).normalize().unwrap();
            let a = 0.5 * (unit_direction.y + 1.0);
            let start_fade = (1.0 - a) * Color::new(1.0, 1.0, 1.0).unwrap();
            let end_fade = a * Color::new(0.5, 0.7, 1.0).unwrap();
            start_fade + end_fade
        }
    }
}

pub fn write(file_handle: &mut std::fs::File, color: Color) {
    let (rbyte, gbyte, bbyte) = color.to_byte_rgb();
    let output_row = format!("{} {} {}\n", rbyte, gbyte, bbyte).into_bytes();
    file_handle.write_all(&output_row).unwrap();
}
