//! Defines an interval in real space from [LOW, HIGH]

use core::f64;

pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Interval {
        Interval { min, max }
    }

    pub fn universe() -> Interval {
        Interval::new(-f64::INFINITY, f64::INFINITY)
    }

    pub fn empty() -> Interval {
        Interval::new(f64::INFINITY, -f64::INFINITY)
    }

    /// Checks if value is in [min, max] (aka inclusive)
    pub fn contains(&self, val: f64) -> bool {
        self.min <= val && val <= self.max
    }

    /// Stricter version of `contains`. Checks for (min, max) (aka exclusive)
    pub fn surrounds(&self, val: f64) -> bool {
        self.min < val && val < self.max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_interval() {
        let i1 = Interval::new(0.1, 1.2);
        assert!(i1.contains(1.0));
        assert!(i1.contains(1.2));
        assert!(i1.contains(0.1));
        assert!(!i1.surrounds(1.2));
        assert!(!i1.surrounds(0.1));
    }

    #[test]
    fn test_empty() {
        let empty_int = Interval::empty();
        let vals_to_check = vec![0.0, 1.0, 100000.0, -100000.0];
        for val in vals_to_check {
            assert!(!empty_int.contains(val));
        }
    }
    #[test]
    fn test_universe() {
        let empty_int = Interval::universe();
        let vals_to_check = vec![0.0, 1.0, 100000.0, -100000.0];
        for val in vals_to_check {
            assert!(empty_int.contains(val));
        }
    }
}
