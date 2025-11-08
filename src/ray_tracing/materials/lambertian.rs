use crate::{
    math_utils::{ray::Ray, vector::Vec3},
    ray_tracing::{color::Color, materials::material::Scatterer, shapes::hittable::HitRecord},
};

/// Lambertian diffuse material

pub struct LambertianMaterial {
    albedo: Color,
}

impl LambertianMaterial {
    pub const fn new(albedo: Color) -> LambertianMaterial {
        LambertianMaterial { albedo }
    }
}

impl Scatterer for LambertianMaterial {
    fn scatter(
        &self,
        r_in: Ray<f64>, // don't need in ray for lambertian reflection
        hit_record: &HitRecord,
    ) -> Option<(Ray<f64>, Color)> {
        let random_unit_vector = Vec3::random_unit_vector_same_hemisphere(&hit_record.normal);
        let scatter_direction = hit_record.normal + random_unit_vector;

        // handle cases where vector is essentially zero by defaulting to the normal
        let scatter_direction = if scatter_direction.near_zero() {
            hit_record.normal
        } else {
            scatter_direction
        };

        let scattered_ray = Ray::new_at_time(hit_record.point, scatter_direction, r_in.get_time());
        let attenuation = self.albedo;

        Some((scattered_ray, attenuation))
    }
}
