//! Group of hittable objects (generic)

use super::hittable::{HitRecord, Hittable};

pub struct Group {
    objects: Vec<Box<dyn Hittable>>,
}

impl Group {
    pub fn new(hittable_objects: Vec<Box<dyn Hittable>>) -> Group {
        Group {
            objects: hittable_objects,
        }
    }
}

impl Hittable for Group {
    fn hit(
        &self,
        ray: &crate::math_utils::ray::Ray<f64>,
        ray_t_min: f64,
        ray_t_max: f64,
    ) -> Option<HitRecord> {
        // O(n) check for each object
        // super inefficient for complex meshes, should use bounding volume hierarchies
        // let mut closest_so_far = ray_t_max;
        let mut hit_record: Option<HitRecord> = None;

        for object in &self.objects {
            match object.hit(ray, ray_t_min, ray_t_max) {
                Some(new_hit_record) => hit_record = Some(new_hit_record),
                None => {}
            }
        }

        hit_record
    }
}
