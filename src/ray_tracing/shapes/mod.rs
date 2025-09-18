
use crate::math_utils::{point::Point, ray::Ray};

pub struct Sphere {
    center: Point<f64>,
    radius: f64
}

impl Sphere {

    pub fn new(center: Point<f64>, radius: f64) -> Sphere {
        Sphere {
            center,
            radius
        }
    }

    pub fn hit_sphere(&self, r: &Ray<f64>) -> f64 {
        let center = self.center;
        let radius = self.radius;

        let oc = center - *r.get_origin();
        let a = r.get_direction().magnitude_squared();
        let h = r.get_direction().dot(&oc);
        let c = oc.magnitude_squared() - radius*radius;
        let discriminant = h*h - a*c;
        
        if discriminant < 0.0 {
            -1.0
        } else {
            return (h - discriminant.sqrt()) / a;
        }
    }
}