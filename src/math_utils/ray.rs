use super::{point::Point, vector::Vec3};
use std::ops::{Add, Mul};
pub struct Ray<T> {
    origin: Point<T>,
    direction: Vec3<T>,
}

impl<T> Ray<T> {
    pub fn new(origin: Point<T>, direction: Vec3<T>) -> Ray<T> {
        Ray {
            origin: origin,
            direction: direction,
        }
    }

    pub fn get_direction(&self) -> &Vec3<T> {
        &self.direction
    }

    pub fn get_origin(&self) -> &Point<T> {
        &self.origin
    }
}

impl<T> Ray<T>
where
    T: Add<Output = T> + Mul<Output = T> + Copy,
{
    pub fn at(&self, time: T) -> Point<T> {
        self.origin + self.direction * time
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ray_at_unit_time() {
        let ray = Ray::new(Point::new(0, 0, 0), Vec3::new(1, 1, 1));
        assert_eq!(ray.at(1), Point::new(1, 1, 1));
    }

    #[test]
    fn test_ray_at_fraciton_time() {
        let ray = Ray::new(Point::new(0., 0., 0.), Vec3::new(0.5, 0.5, 0.5));
        assert_eq!(ray.at(0.5), Point::new(0.25, 0.25, 0.25));
    }
}
