//! The camera class constructs and dispatches rays and uses the results to generate an image

use crate::{
    math_utils::{interval::Interval, point::Point, ray::Ray, vector::Vec3},
    ray_tracing::{
        color::Color,
        shapes::{group::Group, hittable::Hittable},
    },
};
use core::f64;
use indicatif::ProgressIterator;
use std::io::Write;

pub struct Camera {
    center: Point<f64>,
    // aspect_ratio: f64,
    image_width: isize,
    image_height: isize,
    pixel_00_loc: Point<f64>,
    pixel_delta_u: Vec3<f64>,
    pixel_delta_v: Vec3<f64>,
    samples_per_pixel: isize,
    max_depth: isize,
}

impl Default for Camera {
    fn default() -> Self {
        let image_width = 200;
        let aspect_ratio = 1.0;
        let samples_per_pixel = 10;
        let max_depth = 10;

        Camera::new(image_width, aspect_ratio, samples_per_pixel, max_depth)
    }
}

impl Camera {
    pub fn new(
        image_width: isize,
        aspect_ratio: f64,
        samples_per_pixel: isize,
        max_depth: isize,
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
            center: camera_center,
            // aspect_ratio,
            image_height,
            image_width,
            pixel_00_loc: pixel_00_location,
            pixel_delta_u: pixel_du,
            pixel_delta_v: pixel_dv,
            samples_per_pixel,
            max_depth,
        }
    }

    pub fn render(&self, world: Group) {
        // File to write
        let mut file_handle = std::fs::File::create("./image.ppm").unwrap();

        let header = format!("P3\n{} {}\n255\n", self.image_width, self.image_height).into_bytes();
        file_handle.write_all(&header).unwrap();

        let pixel_sample_scale = 1.0 / (self.samples_per_pixel as f64);

        for j in (0..self.image_height).progress() {
            for i in 0..self.image_width {
                let row = j as f64;
                let col = i as f64;

                // let pixel_center =
                //     self.pixel_00_loc + (col * self.pixel_delta_u) + (row * self.pixel_delta_v);
                // let ray_direction = pixel_center - self.center;
                // let ray = Ray::new(self.center, ray_direction);

                // let color = self.get_ray_color(ray, &world);

                let mut color = Color::new(0.0, 0.0, 0.0).unwrap();
                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(col, row);
                    color = color + self.get_ray_color(ray.clone(), &world, self.max_depth);
                }

                write(&mut file_handle, color * pixel_sample_scale);
            }
        }
    }

    fn get_ray_color<T>(&self, ray: Ray<f64>, world: &T, remaining_bounces: isize) -> Color
    where
        T: Hittable,
    {
        const MIN_HIT_DISTANCE: f64 = 0.001;

        if remaining_bounces <= 0 {
            return Color::new(0.0, 0.0, 0.0).unwrap();
        }

        match world.hit(&ray, Interval::new(MIN_HIT_DISTANCE, f64::INFINITY)) {
            Some(hit_record) => {
                let direction = Vec3::random_unit_vector_same_hemisphere(&hit_record.normal);
                let direction = direction + hit_record.normal;
                let bounce_ray = Ray::new(hit_record.point, direction);
                0.5 * self.get_ray_color(bounce_ray, world, remaining_bounces - 1)
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

    fn get_ray(&self, i: f64, j: f64) -> Ray<f64> {
        let offset = Camera::sample_square();
        let ray_origin = self.center;
        let pixel_center = self.pixel_00_loc
            + ((i + offset.x) * self.pixel_delta_u)
            + ((j + offset.y) * self.pixel_delta_v);
        let ray_direction = pixel_center - self.center;

        Ray::new(ray_origin, ray_direction)
    }

    fn sample_square() -> Vec3<f64> {
        Vec3::new(
            rand::random_range(-0.5..0.5),
            rand::random_range(-0.5..0.5),
            0.0,
        )
    }
}

/// Utility function to write a color to the file in RGB format
fn write(file_handle: &mut std::fs::File, color: Color) {
    let (rbyte, gbyte, bbyte) = color.to_byte_rgb();
    let output_row = format!("{} {} {}\n", rbyte, gbyte, bbyte).into_bytes();
    file_handle.write_all(&output_row).unwrap();
}
