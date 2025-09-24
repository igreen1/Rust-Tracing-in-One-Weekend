//! Refractive material

use crate::{
    math_utils::ray::Ray,
    ray_tracing::{color::Color, materials::material::Scatterer, shapes::hittable::HitRecord},
};

pub struct DielectricMaterial {
    refraction_index: f64,
}

impl DielectricMaterial {
    pub const fn new(refraction_index: f64) -> DielectricMaterial {
        DielectricMaterial { refraction_index }
    }
}

impl Scatterer for DielectricMaterial {
    fn scatter(&self, ray_in: Ray<f64>, hit_record: &HitRecord) -> Option<(Ray<f64>, Color)> {
        
        let attenuation = Color::new(1.0, 1.0, 1.0).unwrap();
        let ri = if hit_record.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = ray_in.get_direction().normalize().unwrap();

        let cos_theta = (-unit_direction).dot(&hit_record.normal).min(1.0).max(-1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let direction = if ri * sin_theta > 1.0 {
            // cannot refract
            unit_direction.reflect(&hit_record.normal)
        } else {
            unit_direction.refract(&hit_record.normal, ri)
        };

        let scattered = Ray::new(hit_record.point, direction);

        Some((scattered, attenuation))
    }
}
