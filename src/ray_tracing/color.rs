/// RGB represenation
use std::ops::{Add, Mul, Sub};

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
        let red_byte = (self.red * 255.99).round() as u8;
        let green_byte = (self.green * 255.99).round() as u8;
        let blue_byte = (self.blue * 255.99).round() as u8;
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
            blue: self.blue - rhs.blue
        }
    }
}
impl Add<Color> for Color {
    type Output = Color;
    fn add(self, rhs: Color) -> Self::Output {
        Color {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue
        }
    }
}