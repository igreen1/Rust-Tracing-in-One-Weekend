use num_traits::{Float, Zero};
use std::cmp::{PartialEq, PartialOrd};
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum VectorError {
    InvalidOperation,
}

/// Generic vector in 3 dimensions
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vec3<T>
where
    T: Add<Output = T> + Mul<Output = T> + Sub<Output = T> + Copy + Zero,
{
    /// Create a new vector
    pub fn new(x: T, y: T, z: T) -> Vec3<T> {
        Vec3 { x, y, z }
    }

    /// Calculate the cross product of this vector with another vector
    pub fn cross(&self, other: &Vec3<T>) -> Vec3<T> {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    /// Calculate the vector dot product
    pub fn dot(&self, other: &Vec3<T>) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Calculate the sum of squares of the elements of the 3d vectors
    pub fn magnitude_squared(&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// Return a vector with all elements equal to zero
    pub fn zero() -> Vec3<T> {
        Vec3 {
            x: T::zero(),
            y: T::zero(),
            z: T::zero(),
        }
    }
}

impl<T> Div<T> for Vec3<T>
where
    T: Div<Output = T> + Copy,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl<T> Mul<T> for Vec3<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Self;

    fn mul(self, scalar: T) -> Self::Output {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl Mul<Vec3<f64>> for f64 {
    type Output = Vec3<f64>;

    fn mul(self, vector: Vec3<f64>) -> Self::Output {
        vector * self
    }
}

impl<T: Float> Vec3<T> {
    // Only floats can do magnitude because it returns a float type
    pub fn magnitude(&self) -> T {
        self.magnitude_squared().sqrt()
    }

    /// Prefer normalizing on floats because the values "make sense"
    /// Eg., <1, 1, 0> can't become an integer vector normalized
    pub fn normalize(self) -> Result<Vec3<T>, VectorError> {
        let mag = self.magnitude();
        if mag == T::zero() {
            Err(VectorError::InvalidOperation)
        } else {
            Ok(Vec3::new(self.x / mag, self.y / mag, self.z / mag))
        }
    }
}

impl<T> Add<Vec3<T>> for Vec3<T>
where
    T: Add<Output = T>,
{
    type Output = Vec3<T>;
    fn add(self, rhs: Vec3<T>) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T> Sub for Vec3<T>
where
    T: Sub<Output = T> + Copy + Zero,
{
    type Output = Vec3<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
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
    fn test_vector_creation_f64() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
    }

    #[test]
    fn test_vector_creation_f32() {
        let v = Vec3::new(1.0 as f32, 2.0 as f32, 3.0 as f32);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
    }

    #[test]
    fn test_vector_creation_i32() {
        let v = Vec3::new(1 as i32, 2 as i32, 3 as i32);
        assert_eq!(v.x, 1);
        assert_eq!(v.y, 2);
        assert_eq!(v.z, 3);
    }

    #[test]
    fn test_dot_product() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b: Vec3<f64> = Vec3::new(4.0, -5.0, 6.0);
        let result = a.dot(&b);
        let expected = 1.0 * 4.0 + 2.0 * -5.0 + 3.0 * 6.0; // = 4 -10 +18 = 12.0
        assert_eq!(result, expected);
    }

    #[test]
    fn test_dot_product_orthogonal() {
        let a = Vec3::new(1.0, 0.0, 0.0);
        let b = Vec3::new(0.0, 1.0, 0.0);
        assert_eq!(a.dot(&b), 0.0); // Orthogonal vectors
    }

    #[test]
    fn test_dot_product_parallel() {
        let a = Vec3::new(2.0, 2.0, 2.0);
        let b = Vec3::new(4.0, 4.0, 4.0);
        assert_eq!(a.dot(&b), 2.0 * 4.0 + 2.0 * 4.0 + 2.0 * 4.0); // = 24.0
    }

    #[test]
    fn test_vector_subtraction() {
        let a = Vec3::new(4.0, 5.0, 6.0);
        let b = Vec3::new(1.0, 2.0, 3.0);
        let result = a - b;
        assert_eq!(result, Vec3::new(3.0, 3.0, 3.0));
    }
    #[test]
    fn test_vector_addition() {
        let a = Vec3::new(4.0, 5.0, 6.0);
        let b = Vec3::new(1.0, 2.0, 3.0);
        let result = a + b;
        assert_eq!(result, Vec3::new(5.0, 7.0, 9.0));
    }

    #[test]
    fn test_vector_magnitude() {
        let v = Vec3::new(3.0, 4.0, 0.0);
        assert_eq!(v.magnitude(), 5.0);
    }

    #[test]
    fn test_vector_cross_simple() {
        let u = Vec3::new(0, 1, 0);
        let v = Vec3::new(1, 0, 0);
        let result = u.cross(&v);
        assert_eq!(result, Vec3::new(0, 0, -1));
        let result = v.cross(&u);
        assert_eq!(result, Vec3::new(0, 0, 1));
    }
    #[test]
    fn test_vector_normalization() {
        let v = Vec3::new(3.0, 4.0, 0.0);
        let normalized = v.normalize().unwrap();
        let expected = Vec3::new(0.6, 0.8, 0.0);
        let epsilon = 1e-10;

        assert!((normalized.x - expected.x).abs() < epsilon);
        assert!((normalized.y - expected.y).abs() < epsilon);
        assert!((normalized.z - expected.z).abs() < epsilon);
    }
}
