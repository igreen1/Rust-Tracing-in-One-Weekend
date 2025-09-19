//! Group of hittable objects (generic)

use super::hittable::{HitRecord, Hittable};
use crate::math_utils::interval::Interval;

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
        ray_interval: Interval,
    ) -> Option<HitRecord> {
        // O(n) check for each object
        // super inefficient for complex meshes, should use bounding volume hierarchies
        let mut closest_so_far = ray_interval.max;
        let mut hit_record: Option<HitRecord> = None;

        for object in &self.objects {
            match object.hit(ray, Interval::new(ray_interval.min, closest_so_far)) {
                Some(new_hit_record) => {
                    closest_so_far = new_hit_record.t;
                    hit_record = Some(new_hit_record);
                }
                None => {}
            }
        }

        hit_record
    }
}
