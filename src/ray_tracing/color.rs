/// RGB represenation
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub};

use crate::math_utils::interval::Interval;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum ColorError {
    InvalidColorRange,
}

impl Color {
    pub fn new(red: f64, green: f64, blue: f64) -> Result<Color, ColorError> {
        if red > 1.0 || red < 0. || green > 1. || green < 0. || blue > 1. || blue < 0. {
            Err(ColorError::InvalidColorRange)
        } else {
            Ok(Color { red, green, blue })
        }
    }

    pub fn to_byte_rgb(&self) -> (u8, u8, u8) {
        const COLOR_INTENSITY: Interval = Interval::new(0.000, 1.0);
        let red_byte = (COLOR_INTENSITY.clamp(self.red) * 255.0) as u8;
        let green_byte = (COLOR_INTENSITY.clamp(self.green) * 255.0) as u8;
        let blue_byte = (COLOR_INTENSITY.clamp(self.blue) * 255.0) as u8;
        return (red_byte, green_byte, blue_byte);
    }
}

impl Mul<f64> for Color {
    type Output = Color;
    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            red: self.red * rhs,
            green: self.green * rhs,
            blue: self.blue * rhs,
        }
    }
}

impl Mul<Color> for f64 {
    type Output = Color;
    fn mul(self, rhs: Color) -> Self::Output {
        Color {
            red: self * rhs.red,
            green: self * rhs.green,
            blue: self * rhs.blue,
        }
    }
}

impl Sub<Color> for Color {
    type Output = Color;
    fn sub(self, rhs: Color) -> Self::Output {
        Color {
            red: self.red - rhs.red,
            green: self.green - rhs.green,
            blue: self.blue - rhs.blue,
        }
    }
}
impl Add<Color> for Color {
    type Output = Color;
    fn add(self, rhs: Color) -> Self::Output {
        Color {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
        }
    }
}

impl AddAssign<Color> for Color {
    fn add_assign(&mut self, rhs: Color) {
        self.red += rhs.red;
        self.green += rhs.green;
        self.blue += rhs.blue;
    }
}

impl MulAssign<f64> for Color {
    fn mul_assign(&mut self, rhs: f64) {
        self.red *= rhs;
        self.green *= rhs;
        self.blue *= rhs;
    }
}
