use std::rc::Rc;

use crate::{math_utils::interval::Interval, ray_tracing::materials::material::Scatterer};

pub struct HitRecord {
    /// The point where the ray hit the object
    pub point: crate::math_utils::point::Point<f64>,
    /// The normal vector from the object face at the hit point
    pub normal: crate::math_utils::vector::Vec3<f64>,
    /// The parameter t from the ray equation at the hit point
    /// can be thought of as "time" along the ray
    pub t: f64,
    /// Whether the hit was on the front face of the object
    pub front_face: bool,
    // the material of the object that was hit
    pub material: Rc<dyn Scatterer>,
}

pub trait Hittable {
    fn hit(
        &self,
        ray: &crate::math_utils::ray::Ray<f64>,
        ray_interval: Interval,
    ) -> Option<HitRecord>;
}
