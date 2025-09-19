//! The camera class constructs and dispatches rays and uses the results to generate an image

use indicatif::ProgressIterator;
use std::io::Write;
use crate::{
    math_utils::{interval::Interval, point::Point, ray::Ray, vector::Vec3},
    ray_tracing::{
        color::Color,
        shapes::{group::Group, hittable::Hittable},
    },
};
use core::f64;


pub struct Camera {
    center: Point<f64>,
    aspect_ratio: f64,
    image_width: isize,
    image_height: isize,
    pixel_00_loc: Point<f64>,
    pixel_delta_u: Vec3<f64>,
    pixel_delta_v: Vec3<f64>
}


impl Default for Camera {


    fn default() -> Self {

        let image_width = 100;
        let aspect_ratio = 1.0;

        Camera::new(image_width, aspect_ratio)
    }
}

impl Camera {

    
    pub fn new(
        image_width: isize, aspect_ratio: f64
    ) -> Camera {
        
        
        let image_height = (image_width as f64 / aspect_ratio).round() as isize;
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


        Self {
            center: camera_center, aspect_ratio, image_height, image_width, pixel_00_loc: pixel_00_location, pixel_delta_u: pixel_du, pixel_delta_v: pixel_dv
        }

    }

    pub fn render(&self, world: Group) {

        
        // File to write
        let mut file_handle = std::fs::File::create("./image.ppm").unwrap();

        let header = format!("P3\n{} {}\n255\n", self.image_width, self.image_height).into_bytes();
        file_handle.write_all(&header).unwrap();

        for j in (0..self.image_height).progress() {
            for i in 0..self.image_width {
                let row = j as f64;
                let col = i as f64;

                let pixel_center = self.pixel_00_loc + (col * self.pixel_delta_u) + (row * self.pixel_delta_v);
                let ray_direction = pixel_center - self.center;
                let ray = Ray::new(self.center, ray_direction);

                let color = self.get_ray_color(ray, &world);
                write(&mut file_handle, color);
            }
        }

    }

    fn get_ray_color<T>(&self, ray: Ray<f64>, world: &T) -> Color
    where
        T: Hittable,
    {
        match world.hit(&ray, Interval::new(0.0, f64::INFINITY)) {
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
}



/// Utility function to write a color to the file in RGB format
fn write(file_handle: &mut std::fs::File, color: Color) {
    let (rbyte, gbyte, bbyte) = color.to_byte_rgb();
    let output_row = format!("{} {} {}\n", rbyte, gbyte, bbyte).into_bytes();
    file_handle.write_all(&output_row).unwrap();
}
