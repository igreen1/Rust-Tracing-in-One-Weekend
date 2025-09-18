
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

    // TODO: something is deeply wrong with my z axis
    // pushing objects far away fixes everything ????
    pub fn hit_sphere(&self, ray: &Ray<f64>) -> f64 {
        let orig = *ray.get_origin();
        let oc =  self.center - orig;
        
        let ray_dir = ray.get_direction();//.normalize().unwrap();
        let a: f64 = ray_dir.magnitude_squared(); //ray.get_direction().dot(ray.get_direction());
        let b = -2.0 * ray_dir.dot(&oc);
        let c = oc.magnitude_squared() - self.radius * self.radius;

        let discriminant = b*b - 4.0*a*c;
        
        if discriminant < 0.0 {
            -1.0
        } else {
            // return 1.0;
            let sqrt_disc = discriminant.sqrt();
            let t1 = (-b - sqrt_disc) / (2.0 * a);
            return t1;
        }
    }
}