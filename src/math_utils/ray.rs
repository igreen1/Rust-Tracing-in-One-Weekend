
use std::ops::{Add, Mul}
use super::{point::Point, vector::Vec3};
pub struct Ray<T> {
    origin: Point<T>,
    direction: Vec3<T>,
}

// impl Ray<f64> {
//     pub fn at(&self, time: f64) -> Point<f64> {
//         self.origin + self.direction * time
//     }
// }

impl<T> Ray<T> 
where
    T: Add<Output = T> + Mul<Output = T> + Copy

{
    pub fn at(&self, time: T) -> Point<T> {
        self.origin + self.direction * time
    }
}
