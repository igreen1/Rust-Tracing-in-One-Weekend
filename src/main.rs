use crate::math_utils::point::Point;
use crate::math_utils::ray::Ray;
use crate::math_utils::vector::Vec3;
use crate::ray_tracing::color::Color;
use indicatif::ProgressIterator;
use std::io::Write;
pub mod math_utils;
pub mod ray_tracing;
use ray_tracing::shapes::Sphere;

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

    let viewport_upper_left =camera_center
                             - Vec3::new(0., 0., focal_length) - viewport_u/2. - viewport_v/2.;
    let pixel_00_location = viewport_upper_left + 0.5 * (pixel_du + pixel_dv);
    // *** Start Rendering ***

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

            let color = get_ray_color(ray);
            write(&mut file_handle, color);
        }
    }
}

fn get_ray_color(ray: Ray<f64>) -> Color {

    let sphere = Sphere::new(
        // in right hand coordinate system, "z is negative into the screen"
        // so as we go further away, z should get more negative
        // so somewhere, my zs are fucked up
        Point::new(0.0, 0.0, -1.), 
        0.5
    );
    let t = sphere.hit_sphere(&ray);
    if t > 0.0{
        return Color::new(1.0, 0.0, 0.0).unwrap();

        // let N = (ray.at(t) - Vec3::new(0.0, 0.0, -1.0));
        // let norm_vec = Vec3::new(N.x, N.y, N.z).normalize().unwrap();
        // return 0.5 * Color::new(
        //     norm_vec.x + 1.0,
        //     norm_vec.y + 1.0,
        //     norm_vec.z + 1.0
        // ).unwrap()
    }
    let unit_direction = (*ray.get_direction()).normalize().unwrap();

    let a = 0.5 * (unit_direction.y + 1.0);
    let start_fade = (1.0 - a) * Color::new(1.0, 1.0, 1.0).unwrap();
    let end_fade = a * Color::new(0.5, 0.7, 1.0).unwrap();
    start_fade + end_fade

}

pub fn write(file_handle: &mut std::fs::File, color: Color) {
    let (rbyte, gbyte, bbyte) = color.to_byte_rgb();
    let output_row = format!("{} {} {}\n", rbyte, gbyte, bbyte).into_bytes();
    file_handle.write_all(&output_row).unwrap();
}
