use super::vector::Vec3;
use std::cmp::{PartialEq, PartialOrd};
use std::ops::{Add, Sub};

/// Point in XYZ space
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Point<T> {
    /// Create a new point in 3d space
    pub fn new(x: T, y: T, z: T) -> Point<T> {
        Point { x, y, z }
    }
}

impl<T> Sub<Vec3<T>> for Point<T>
where
    T: Sub<Output = T>,
{
    type Output = Point<T>;
    fn sub(self, rhs: Vec3<T>) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T> Add<Vec3<T>> for Point<T>
where
    T: Add<Output = T>,
{
    type Output = Point<T>;
    fn add(self, rhs: Vec3<T>) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T> Add<Point<T>> for Vec3<T>
where
    T: Add<Output = T>,
{
    type Output = Point<T>;
    fn add(self, rhs: Point<T>) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T> Sub for Point<T>
where
    T: Sub<Output = T>,
{
    type Output = Vec3<T>;

    /// Create a vector from the two points by subtraction
    fn sub(self, rhs: Self) -> Vec3<T> {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_subtraction() {
        let p1 = Point::new(1, 0, 0);
        let p2 = Point::new(1, 1, 1);
        let v = p1 - p2;
        assert_eq!(v, Vec3::new(0, -1, -1));
        let v = p2 - p1;
        assert_eq!(v, Vec3::new(0, 1, 1));
    }

    #[test]
    fn test_point_vector_addition() {
        let p1 = Point::new(1.0, 1.0, 1.0);
        let u = Vec3::new(3.0, 4.0, 5.0);
        let result = p1 + u;
        assert_eq!(result, Point::new(4.0, 5.0, 6.0));
    }
}
