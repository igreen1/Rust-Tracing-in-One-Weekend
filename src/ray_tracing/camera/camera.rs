//! The camera class constructs and dispatches rays and uses the results to generate an image

use crate::{
    math_utils::{interval::Interval, point::Point, ray::Ray, vector::Vec3},
    ray_tracing::{
        color::Color,
        shapes::{group::Group, hittable::Hittable},
    },
};
use core::f64;
use indicatif::ParallelProgressIterator;
use rayon::prelude::*;
use std::io::Write;
use std::sync::Arc;

#[derive(Clone, Copy)]
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
    defocus_dist_u: Vec3<f64>,
    defocus_dist_v: Vec3<f64>,
    defocus_angle: f64,
    // basis_vectors: Vec3<Vec3<f64>>,
    // vertical_fov: f64,
}

/// Make it a bit easier to create a camera by letting a user
/// set some elements in a stream-like manner
pub struct CameraBuilder {
    pub image_width: isize,
    pub aspect_ratio: f64,
    pub samples_per_pixel: isize,
    pub max_depth: isize,
    pub vertical_fov: f64,
    pub lookfrom: Point<f64>,
    pub lookat: Point<f64>,
    pub defocus_angle: f64,
    pub focus_dist: f64,
}

impl CameraBuilder {
    pub fn make_camera(&self) -> Camera {
        Camera::new(
            self.image_width,
            self.aspect_ratio,
            self.samples_per_pixel,
            self.max_depth,
            self.vertical_fov,
            self.lookfrom,
            self.lookat,
            self.defocus_angle,
            self.focus_dist,
        )
    }

    pub fn set_look_at(&self, look_at: Point<f64>) -> Self {
        CameraBuilder {
            lookat: look_at,
            ..(*self)
        }
    }

    pub fn set_look_from(&self, x: f64, y: f64, z: f64) -> Self {
        let look_from = Point::new(x, y, z);
        CameraBuilder {
            lookfrom: look_from,
            ..(*self)
        }
    }

    pub fn set_defocus_angle(&self, defocus_angle: f64) -> Self {
        CameraBuilder {
            defocus_angle,
            ..(*self)
        }
    }

    pub fn set_focus_dist(&self, focus_dist: f64) -> Self {
        CameraBuilder {
            focus_dist,
            ..(*self)
        }
    }
}

impl Default for CameraBuilder {
    fn default() -> Self {
        let image_width = 500;
        let aspect_ratio = 16.0 / 9.0;
        let samples_per_pixel = 100;
        let max_depth = 50;

        let vertical_fov: f64 = 20.;

        // flat looking along the z axis
        let lookfrom = Point::<f64>::new(0., 0., 0.);
        let lookat = Point::<f64>::new(0., 0., -1.0);

        let defocus_angle = 0.0;
        let focus_dist = 10.0;

        CameraBuilder {
            image_width,
            aspect_ratio,
            samples_per_pixel,
            max_depth,
            vertical_fov,
            lookfrom,
            lookat,
            defocus_angle,
            focus_dist,
        }
    }
}

impl Default for Camera {
    fn default() -> Self {
        CameraBuilder::default().make_camera()
    }
}

impl Camera {
    pub fn new(
        image_width: isize,
        aspect_ratio: f64,
        samples_per_pixel: isize,
        max_depth: isize,
        vertical_fov: f64,
        lookfrom: Point<f64>,
        lookat: Point<f64>,
        defocus_angle: f64,
        focus_dist: f64,
    ) -> Camera {
        let image_height = (image_width as f64 / aspect_ratio).round() as isize;
        // clamp height to 1 at a minimum
        let image_height = if image_height < 1 { 1 } else { image_height };

        // create the viewport
        let image_aspect_ratio = image_width as f64 / image_height as f64;

        let theta = vertical_fov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focus_dist;

        let viewport_width = viewport_height * image_aspect_ratio;

        // make the camera basis vectors
        let vup = Vec3::<f64>::new(0., 1., 0.);
        let w = (lookfrom - lookat).normalize().unwrap();
        let u = vup.cross(&w).normalize().unwrap();
        let v = w.cross(&u); //.normalize().unwrap();

        let center = lookfrom;

        // setup the viewport with the unit vectors
        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        let pixel_du = viewport_u / (image_width as f64);
        let pixel_dv = viewport_v / (image_height as f64);

        let viewport_upper_left = center - (focus_dist * w) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel_00_location = viewport_upper_left + 0.5 * (pixel_du + pixel_dv);

        let defocus_radius = focus_dist * (defocus_angle / 2.0).to_radians().tan();
        let defocus_dist_u = u * defocus_radius;
        let defocus_dist_v = v * defocus_radius;

        Self {
            center: center,
            // aspect_ratio,
            image_height,
            image_width,
            pixel_00_loc: pixel_00_location,
            pixel_delta_u: pixel_du,
            pixel_delta_v: pixel_dv,
            samples_per_pixel,
            max_depth,
            defocus_dist_u,
            defocus_dist_v,
            defocus_angle, // basis_vectors: Vec3 { x: u, y: v, z: w }
        }
    }

    pub fn render(&self, world: Group) {
        let pixel_sample_scale = 1.0 / (self.samples_per_pixel as f64);
        let world_rc = Arc::new(world);

        // main Render loop!
        // All done in parallel, so each row is performed in parallel using rayon
        let pixels: Vec<Vec<Color>> = (0..self.image_height)
            .into_par_iter()
            // And get a nice progress bar!
            .progress_count(self.image_height.try_into().unwrap())
            .map(|j| {
                // now, for each column in the row
                (0..self.image_width)
                    .map(|i| {
                        let row = j as f64;
                        let col = i as f64;

                        // get a black color
                        let mut color = Color::new(0.0, 0.0, 0.0).unwrap();

                        // then for each pixel, take a bunch of samples and add what we see
                        for _ in 0..self.samples_per_pixel {
                            let ray = self.get_ray(col, row);
                            color = color
                                + self.get_ray_color(
                                    ray.clone(),
                                    Arc::clone(&world_rc),
                                    self.max_depth,
                                );
                        }
                        // scale because we do't want to overly sample a pixel
                        color * pixel_sample_scale
                    })
                    .collect()
            })
            .collect(); // get everything back into the maian thread

        // now that we're back in the main thread, open and edit our file
        let mut file_handle = std::fs::File::create("./image.ppm").unwrap();
        // for the PPM formt, write a header
        let header = format!("P3\n{} {}\n255\n", self.image_width, self.image_height).into_bytes();
        file_handle.write_all(&header).unwrap();

        // write each pixel as RGB values for PPM formt
        pixels.into_iter().for_each(|row| {
            row.into_iter()
                .for_each(|pixel_col| write(&mut file_handle, pixel_col));
        });
    }

    fn get_ray_color<T>(&self, ray: Ray<f64>, world: Arc<T>, remaining_bounces: isize) -> Color
    where
        T: Hittable,
    {
        const MIN_HIT_DISTANCE: f64 = 0.01;

        if remaining_bounces <= 0 {
            return Color::new(0.0, 0.0, 0.0).unwrap();
        }

        match world.hit(&ray, Interval::new(MIN_HIT_DISTANCE, f64::INFINITY)) {
            // if we hit something
            Some(hit_record) => {
                // then scatter off that something
                match hit_record.material.scatter(ray, &hit_record) {
                    // if the scatterer produces a valid scatter
                    Some((scattered_ray, attenuation)) => {
                        let scatter_result =
                            self.get_ray_color(scattered_ray, world, remaining_bounces - 1);
                        Color::new(
                            attenuation.red * scatter_result.red,
                            attenuation.green * scatter_result.green,
                            attenuation.blue * scatter_result.blue,
                        )
                        .unwrap()
                    }
                    // else no way to scatter so return black
                    None => Color::new(0.0, 0.0, 0.0).unwrap(),
                }
            }
            // hit nothing, so grab thge background color (diffuse light source)
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
        // let ray_origin = self.center;
        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let pixel_center = self.pixel_00_loc
            + ((i + offset.x) * self.pixel_delta_u)
            + ((j + offset.y) * self.pixel_delta_v);
        let ray_direction = pixel_center - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn defocus_disk_sample(&self) -> Point<f64> {
        let p = Vec3::random_in_unit_dist();

        self.center + (p.x * self.defocus_dist_u) + (p.y * self.defocus_dist_v)
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
    let (rbyte, gbyte, bbyte) = color.to_bytes_rgb();
    let output_row = format!("{} {} {}\n", rbyte, gbyte, bbyte).into_bytes();
    file_handle.write_all(&output_row).unwrap();
}
