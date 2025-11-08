pub mod interval;
pub mod point;
pub mod ray;
pub mod vector;


pub fn degrees_to_radians(angle_in_degrees: f64) -> f64 {
    angle_in_degrees * std::f64::consts::PI / 180.0
}