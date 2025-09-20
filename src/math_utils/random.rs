//! Convenience wrappers for random number generation.

pub const fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.
}
