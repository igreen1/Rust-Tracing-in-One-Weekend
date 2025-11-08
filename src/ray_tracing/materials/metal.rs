use crate::{
    math_utils::ray::Ray,
    ray_tracing::{color::Color, materials::material::Scatterer, shapes::hittable::HitRecord},
};

pub struct MetalMaterial {
    albedo: Color,
}

impl MetalMaterial {
    pub const fn new(albedo: Color) -> MetalMaterial {
        MetalMaterial { albedo }
    }
}

impl Scatterer for MetalMaterial {
    fn scatter(&self, ray_in: Ray<f64>, hit_record: &HitRecord) -> Option<(Ray<f64>, Color)> {
        let reflection_direction = ray_in.get_direction().reflect(&hit_record.normal);
        let scattered_ray = Ray::new_at_time(hit_record.point, reflection_direction, ray_in.get_time());
        let attenuation = self.albedo;

        Some((scattered_ray, attenuation))
    }
}
