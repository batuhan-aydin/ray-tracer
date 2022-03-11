use std::ops::{Add, Sub, Mul};

use crate::helper::equal;

#[derive(Debug, Copy, Clone)]
pub struct Color {
    red: f32,
    green: f32,
    blue: f32
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        equal(self.red, other.red) && 
        equal(self.green, other.green) && 
        equal(self.blue, other.blue)
    }
}

impl Color {
    pub fn new(red: f32, green: f32, blue: f32) -> Color {
        Color { red, green, blue }
    }

    pub fn hadamard_product(&self, other: &Color) -> Color {
        Color { red: self.red * other.red, 
                green: self.green * other.green, 
                blue: self.blue * other.blue }
    }

    pub fn to_ppm_format(&self) -> String {
        format!("{} {} {}", Color::scale_the_color(self.red),
                            Color::scale_the_color(self.green),
                            Color::scale_the_color(self.blue))
    }

    fn scale_the_color(input: f32) -> u8 {
        if input >= 1.0 {
            return 255;
        } else if input <= 0.0 {
            return 0;
        } else {
            (255.0 * input) as u8
        }
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, rhs: Self) -> Self::Output {
        Color {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue
        }
    }
}

impl Sub for Color {
    type Output = Color;

    fn sub(self, rhs: Self) -> Self::Output {
        Color {
            red: self.red - rhs.red,
            green: self.green - rhs.green,
            blue: self.blue - rhs.blue
        }
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, rhs: Self) -> Self::Output {
        Color {
            red: self.red * rhs.red,
            green: self.green * rhs.green,
            blue: self.blue * rhs.blue
        }
    }
}

impl Mul<Color> for f32 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Color {
        Color { red: self * rhs.red, green: self * rhs.green, blue: self * rhs.blue }
    }
}

impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, rhs: f32) -> Color {
        Color { red: self.red * rhs, green: self.green * rhs, blue: self.blue * rhs }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adding_colors_is_correct() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        let expected = Color::new(1.6, 0.7, 1.0);
        let result = c1 + c2;
        assert_eq!(result, expected);
    }

    #[test]
    fn subtracting_colors_is_correct() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        let expected = Color::new(0.2, 0.5, 0.5);
        let result = c1 - c2;
        assert_eq!(result, expected);
    }

    #[test]
    fn multiplying_color_with_f32_is_correct() {
        let c = Color::new(0.2, 0.3, 0.4);
        let expected = Color::new(0.4, 0.6, 0.8);
        let result = c * 2.0;
        assert_eq!(result, expected);
    }

    #[test]
    fn multiplying_colors_is_correct() {
        let c1 = Color::new(1.0, 0.2, 0.4);
        let c2 = Color::new(0.9, 1.0, 0.1);
        let expected = Color::new(0.9, 0.2, 0.04);
        let result = c1 * c2;
        assert_eq!(result, expected);
    }
}