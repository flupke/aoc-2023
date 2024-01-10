use std::ops::{Add, AddAssign, Neg, Sub};

use super::array::Coordinate;

#[derive(Debug, Clone, Default, PartialEq, Hash, Eq, Copy)]
pub struct Vector {
    pub x: i32,
    pub y: i32,
}

impl Vector {
    pub fn manhattan_distance(&self, other: &Vector) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    pub fn rotate_clockwise(&self) -> Self {
        Vector {
            x: -self.y,
            y: self.x,
        }
    }

    pub fn rotate_counterclockwise(&self) -> Self {
        Vector {
            x: self.y,
            y: -self.x,
        }
    }
}

impl Coordinate for &Vector {
    fn x(&self) -> usize {
        self.x as usize
    }

    fn y(&self) -> usize {
        self.y as usize
    }
}

impl Coordinate for Vector {
    fn x(&self) -> usize {
        self.x as usize
    }

    fn y(&self) -> usize {
        self.y as usize
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, other: Vector) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self {
        Vector {
            x: -self.x,
            y: -self.y,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate() {
        assert_eq!(
            Vector { x: 1, y: 0 }.rotate_clockwise(),
            Vector { x: 0, y: 1 }
        );
        assert_eq!(
            Vector { x: 1, y: 0 }.rotate_counterclockwise(),
            Vector { x: 0, y: -1 }
        );
    }
}
