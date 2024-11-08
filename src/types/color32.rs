use crate::types::color::Color;
use std::fmt;
use std::ops;

/// A type that holds the red, green, blue and alpha channel of a color (in 32 bit format)
pub struct Color32 {
    pub r: u32,
    pub g: u32,
    pub b: u32,
    pub a: u32,
}

impl fmt::Display for Color32 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "({}, {}, {}, {})",
            self.r, self.g, self.b, self.a
        )
    }
}

impl Copy for Color32 {}

impl Clone for Color32 {
    fn clone(&self) -> Color32 {
        *self
    }
}

// Mathematical operator overloading for Color32 type on Color32 type
impl ops::Add<Color32> for Color32 {
    type Output = Color32;
    fn add(self, rhs: Color32) -> Color32 {
        return Color32 {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
            a: self.a + rhs.a,
        };
    }
}

impl ops::Sub<Color32> for Color32 {
    type Output = Color32;
    fn sub(self, rhs: Color32) -> Color32 {
        return Color32 {
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b,
            a: self.a - rhs.a,
        };
    }
}

impl ops::Mul<Color32> for Color32 {
    type Output = Color32;
    fn mul(self, rhs: Color32) -> Color32 {
        return Color32 {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
            a: self.a * rhs.a,
        };
    }
}

impl ops::Div<Color32> for Color32 {
    type Output = Color32;
    fn div(self, rhs: Color32) -> Color32 {
        return Color32 {
            r: self.r / rhs.r,
            g: self.g / rhs.g,
            b: self.b / rhs.b,
            a: self.a / rhs.a,
        };
    }
}

// Mathematical operator overloading for Color32 type on u32 type
impl ops::Add<u32> for Color32 {
    type Output = Color32;
    fn add(self, rhs: u32) -> Color32 {
        return Color32 {
            r: self.r + rhs,
            g: self.g + rhs,
            b: self.b + rhs,
            a: self.a + rhs,
        };
    }
}

impl ops::Sub<u32> for Color32 {
    type Output = Color32;
    fn sub(self, rhs: u32) -> Color32 {
        return Color32 {
            r: self.r - rhs,
            g: self.g - rhs,
            b: self.b - rhs,
            a: self.a - rhs,
        };
    }
}

impl ops::Mul<u32> for Color32 {
    type Output = Color32;
    fn mul(self, rhs: u32) -> Color32 {
        return Color32 {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
            a: self.a * rhs,
        };
    }
}

impl ops::Div<u32> for Color32 {
    type Output = Color32;
    fn div(self, rhs: u32) -> Color32 {
        return Color32 {
            r: self.r / rhs,
            g: self.g / rhs,
            b: self.b / rhs,
            a: self.a / rhs,
        };
    }
}

impl ops::AddAssign for Color32 {
    fn add_assign(&mut self, rhs: Color32) {
        *self = Color32 {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
            a: self.a + rhs.a,
        }
    }
}

impl ops::SubAssign for Color32 {
    fn sub_assign(&mut self, rhs: Color32) {
        *self = Color32 {
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b,
            a: self.a - rhs.a,
        }
    }
}

impl ops::MulAssign for Color32 {
    fn mul_assign(&mut self, rhs: Color32) {
        *self = Color32 {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
            a: self.a * rhs.a,
        }
    }
}

impl ops::DivAssign for Color32 {
    fn div_assign(&mut self, rhs: Color32) {
        *self = Color32 {
            r: self.r / rhs.r,
            g: self.g / rhs.g,
            b: self.b / rhs.b,
            a: self.a / rhs.a,
        }
    }
}

// Logical operator overloading for type Color32
impl PartialEq for Color32 {
    fn eq(&self, other: &Color32) -> bool {
        return self.r == other.r && self.g == other.g && self.b == other.b && self.a == other.a;
    }
    fn ne(&self, other: &Color32) -> bool {
        return self.r != other.r || self.g != other.g || self.b != other.b || self.a != other.a;
    }
}

impl Color32 {
    // Default constructor to initialize Color32
    pub fn new() -> Self {
        return Color32 {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        };
    }

    // Convert color format from HEX to RGB
    pub fn from_hex(hex: &str) -> Self {
        return if hex.len() != 6 {
            Color32 {
                r: 0,
                g: 0,
                b: 0,
                a: 255,
            }
        } else {
            let r = u32::from_str_radix(&hex[0..2], 16).unwrap();
            let g = u32::from_str_radix(&hex[2..4], 16).unwrap();
            let b = u32::from_str_radix(&hex[4..6], 16).unwrap();

            Color32 { r, g, b, a: 255 }
        };
    }

    // Unit colors
    pub fn red() -> Self {
        return Color32 {
            r: 255,
            g: 0,
            b: 0,
            a: 255,
        };
    }
    pub fn green() -> Self {
        return Color32 {
            r: 0,
            g: 255,
            b: 0,
            a: 255,
        };
    }
    pub fn blue() -> Self {
        return Color32 {
            r: 0,
            g: 0,
            b: 255,
            a: 255,
        };
    }
    pub fn yellow() -> Self {
        return Color32 {
            r: 255,
            g: 255,
            b: 0,
            a: 255,
        };
    }
    pub fn pink() -> Self {
        return Color32 {
            r: 255,
            g: 0,
            b: 255,
            a: 255,
        };
    }
    pub fn teal() -> Self {
        return Color32 {
            r: 0,
            g: 255,
            b: 255,
            a: 255,
        };
    }
    pub fn black() -> Self {
        return Color32 {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        };
    }
    pub fn white() -> Self {
        return Color32 {
            r: 255,
            g: 255,
            b: 255,
            a: 255,
        };
    }
    pub fn transparent() -> Self {
        return Self {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        };
    }

    /// Color normalized so that its RGBA values are between 0 and 255
    pub fn normalized(&mut self) -> Color32 {
        return Color32 {
            r: if self.r > 255 { 255 } else { self.r },
            g: if self.g > 255 { 255 } else { self.g },
            b: if self.b > 255 { 255 } else { self.b },
            a: if self.a > 255 { 255 } else { self.a },
        };
    }

    /// Covert color32 to color
    pub fn to_color(&self) -> Color {
        return Color {
            r: (self.r / 255) as f32,
            g: (self.g / 255) as f32,
            b: (self.b / 255) as f32,
            a: (self.a / 255) as f32,
        };
    }
}
