//! A sphere shape for ray tracing

use std::sync::Arc;

use super::hittable::{HitRecord, Hittable};
use crate::{
    math_utils::{interval::Interval, point::Point, ray::Ray, vector::Vec3},
    ray_tracing::materials::material::Scatterer,
};

pub struct Sphere {
    center: Point<f64>,
    radius: f64,
    material: Arc<dyn Scatterer + Sync + Send>, // TODO: change to type generic with constraint
    velocity: Vec3<f64>,
}


impl Sphere {
    pub fn new(center: Point<f64>, radius: f64, material: Arc<dyn Scatterer + Send + Sync>) -> Sphere {
        let velocity = Vec3::<f64>::zero();
        Sphere {
            center,
            radius,
            material,
            velocity
        }
    }

    pub const fn new_with_velocity(center: Point<f64>, radius: f64, material: Arc<dyn Scatterer + Send + Sync>, velocity: Vec3<f64>) -> Sphere {
        Sphere {
            center,
            radius,
            material,
            velocity
        }
    }

    pub const fn get_center(&self) -> Point<f64> {
        self.center
    }
    pub const fn get_radius(&self) -> f64 {
        self.radius
    }

    pub fn at(&self, time: f64) -> Point<f64> {
        return self.center + (time * self.velocity)        
    }

    pub fn add_velocity(self, velocity: Vec3<f64>) -> Sphere {
        Sphere {
            velocity,
            ..self
        }
    }

}

impl Hittable for Sphere {
    fn hit(
        &self,
        ray: &Ray<f64>,
        ray_interval: Interval,
    ) -> Option<crate::ray_tracing::shapes::hittable::HitRecord> {
        // solve our quadratic equation
        let center = self.at(ray.get_time());
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
        let mut t = (h - sqrtd) / a;
        if !ray_interval.surrounds(t) {
            t = (h + sqrtd) / a;
            if !ray_interval.surrounds(t) {
                return None;
            }
        }

        let point = ray.at(t);
        let outward_normal = (point - center) / self.radius;
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
            material: Arc::clone(&self.material) // self.material.clone(),
        })
    }
}

