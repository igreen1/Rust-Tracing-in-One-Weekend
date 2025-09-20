//! A sphere shape for ray tracing

use super::hittable::{HitRecord, Hittable};
use crate::math_utils::{interval::Interval, point::Point, ray::Ray};

pub struct Sphere {
    center: Point<f64>,
    radius: f64,
}

impl Sphere {
    pub const fn new(center: Point<f64>, radius: f64) -> Sphere {
        Sphere { center, radius }
    }

    pub const fn get_center(&self) -> Point<f64> {
        self.center
    }
    pub const fn get_radius(&self) -> f64 {
        self.radius
    }
}

impl Hittable for Sphere {
    fn hit(
        &self,
        ray: &Ray<f64>,
        ray_interval: Interval,
    ) -> Option<crate::ray_tracing::shapes::hittable::HitRecord> {
        // solve our quadratic equation
        let center = self.center;
        let radius = self.radius;

        let oc = center - *ray.get_origin();
        let a = ray.get_direction().magnitude_squared();
        let h = ray.get_direction().dot(&oc);
        let c = oc.magnitude_squared() - radius * radius;
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return None;
        }

        // check which root is within the acceptable range, if any
        let sqrtd = discriminant.sqrt();
        let t = (h - sqrtd) / a;
        if !ray_interval.surrounds(t) {
            let t = (h + sqrtd) / a;
            if !ray_interval.surrounds(t) {
                return None;
            }
        }

        let point = ray.at(t);
        let outward_normal = (point - self.center) / self.radius;
        let front_face = ray.get_direction().dot(&outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        Some(HitRecord {
            point,
            normal,
            t,
            front_face,
        })
    }
}
