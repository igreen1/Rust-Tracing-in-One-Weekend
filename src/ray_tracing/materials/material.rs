use crate::{
    math_utils::ray::Ray,
    ray_tracing::{color::Color, shapes::hittable::HitRecord},
};

pub trait Scatterer: Sync {
    fn scatter(&self, ray_in: Ray<f64>, hit_record: &HitRecord) -> Option<(Ray<f64>, Color)>;
}
