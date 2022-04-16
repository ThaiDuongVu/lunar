use crate::types::vector2::Vector2;
use std::fmt;
use std::ops;

/// A type that holds 2 values (x and y)
pub struct Vector2Int {
    pub x: i32,
    pub y: i32,
}

impl fmt::Display for Vector2Int {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "({}, {})", self.x, self.y)
    }
}

impl Copy for Vector2Int {}

impl Clone for Vector2Int {
    fn clone(&self) -> Vector2Int {
        *self
    }
}

// Mathematical operator overloading for Vector type on Vector type
impl ops::Add<Vector2Int> for Vector2Int {
    type Output = Vector2Int;
    fn add(self, rhs: Vector2Int) -> Vector2Int {
        return Vector2Int {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        };
    }
}

impl ops::Sub<Vector2Int> for Vector2Int {
    type Output = Vector2Int;
    fn sub(self, rhs: Vector2Int) -> Vector2Int {
        return Vector2Int {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        };
    }
}

impl ops::Mul<Vector2Int> for Vector2Int {
    type Output = Vector2Int;
    fn mul(self, rhs: Vector2Int) -> Vector2Int {
        return Vector2Int {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        };
    }
}

impl ops::Div<Vector2Int> for Vector2Int {
    type Output = Vector2Int;
    fn div(self, rhs: Vector2Int) -> Vector2Int {
        return Vector2Int {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        };
    }
}

impl ops::AddAssign for Vector2Int {
    fn add_assign(&mut self, rhs: Vector2Int) {
        *self = Vector2Int {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::SubAssign for Vector2Int {
    fn sub_assign(&mut self, rhs: Vector2Int) {
        *self = Vector2Int {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::MulAssign for Vector2Int {
    fn mul_assign(&mut self, rhs: Vector2Int) {
        *self = Vector2Int {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl ops::DivAssign for Vector2Int {
    fn div_assign(&mut self, rhs: Vector2Int) {
        *self = Vector2Int {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl ops::Neg for Vector2Int {
    type Output = Vector2Int;
    fn neg(self) -> Vector2Int {
        return Vector2Int {
            x: -self.x,
            y: -self.y,
        };
    }
}

// Mathematical operator overloading for Vector type on i32 type
impl ops::Add<i32> for Vector2Int {
    type Output = Vector2Int;
    fn add(self, rhs: i32) -> Vector2Int {
        return Vector2Int {
            x: self.x + rhs,
            y: self.y + rhs,
        };
    }
}

impl ops::Sub<i32> for Vector2Int {
    type Output = Vector2Int;
    fn sub(self, rhs: i32) -> Vector2Int {
        return Vector2Int {
            x: self.x - rhs,
            y: self.y - rhs,
        };
    }
}

impl ops::Mul<i32> for Vector2Int {
    type Output = Vector2Int;
    fn mul(self, rhs: i32) -> Vector2Int {
        return Vector2Int {
            x: self.x * rhs,
            y: self.y * rhs,
        };
    }
}

impl ops::Div<i32> for Vector2Int {
    type Output = Vector2Int;
    fn div(self, rhs: i32) -> Vector2Int {
        return Vector2Int {
            x: self.x / rhs,
            y: self.y / rhs,
        };
    }
}

impl ops::AddAssign<i32> for Vector2Int {
    fn add_assign(&mut self, rhs: i32) {
        *self = Vector2Int {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

impl ops::SubAssign<i32> for Vector2Int {
    fn sub_assign(&mut self, rhs: i32) {
        *self = Vector2Int {
            x: self.x - rhs,
            y: self.y - rhs,
        }
    }
}

// Logical operator overloading for type Vector
impl PartialEq for Vector2Int {
    fn eq(&self, other: &Vector2Int) -> bool {
        return self.x == other.x && self.y == other.y;
    }

    fn ne(&self, other: &Vector2Int) -> bool {
        return self.x != other.x || self.y != other.y;
    }
}

impl Vector2Int {
    /// Default constructor to initialize Vector2Int
    pub fn new() -> Self {
        return Self { x: 0, y: 0 };
    }

    // Unit vectors
    pub fn zero() -> Self {
        return Self { x: 0, y: 0 };
    }
    pub fn identity() -> Self {
        return Self { x: 1, y: 1 };
    }
    pub fn unit_x() -> Self {
        return Self { x: 1, y: 0 };
    }
    pub fn unit_y() -> Self {
        return Self { x: 0, y: 1 };
    }

    // Directional vectors
    pub fn up() -> Self {
        return Self { x: 0, y: 1 };
    }
    pub fn down() -> Self {
        return Self { x: 0, y: -1 };
    }
    pub fn left() -> Self {
        return Self { x: -1, y: 0 };
    }
    pub fn right() -> Self {
        return Self { x: 1, y: 0 };
    }

    /// Return length of current vector
    pub fn length(&mut self) -> f32 {
        return ((self.x.pow(2) + self.y.pow(2)) as f32).sqrt();
    }

    /// Normalize vector so that its length is 1
    /// Note: Return type is Vector2
    pub fn normalized(&mut self) -> Vector2 {
        if self.x == 0 && self.y == 0 {
            return Vector2::zero();
        }
        return Vector2 {
            x: (1 as f32 / self.length()) * (self.x as f32),
            y: (1 as f32 / self.length()) * (self.y as f32),
        };
    }

    /// Return distance from current vector to another vector
    pub fn distance(&mut self, other: Vector2Int) -> f32 {
        return (((self.x - other.x).pow(2) + (self.y - other.y).pow(2)) as f32).sqrt();
    }

    /// Return dot product of current vector and another vector
    pub fn dot(&mut self, other: Vector2Int) -> i32 {
        return self.x * other.x + self.y * other.y;
    }

    /// Translate current vector with another vector
    pub fn translate(&mut self, delta: Vector2Int) {
        self.x += delta.x;
        self.y += delta.y;
    }
}
