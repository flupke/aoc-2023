use std::{
    ops::{Add, AddAssign, Div, Neg, Sub},
    usize,
};

use num_traits::Float;

use super::grid::Coordinate;

#[derive(Debug, Clone, Default, PartialEq, Hash, Eq, Copy, Ord, PartialOrd)]
pub struct Vector<T = i32> {
    pub x: T,
    pub y: T,
}

impl<T: num_traits::Signed + Copy> Vector<T> {
    pub fn new(x: T, y: T) -> Self {
        Vector { x, y }
    }

    pub fn manhattan_distance(&self, other: &Self) -> T {
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

impl Coordinate for Vector<i32> {
    fn new(x: usize, y: usize) -> Self {
        Vector {
            x: x as i32,
            y: y as i32,
        }
    }

    fn x(&self) -> usize {
        self.x as usize
    }

    fn y(&self) -> usize {
        self.y as usize
    }
}

impl Coordinate for Vector<i16> {
    fn new(x: usize, y: usize) -> Self {
        Vector {
            x: x as i16,
            y: y as i16,
        }
    }

    fn x(&self) -> usize {
        self.x as usize
    }

    fn y(&self) -> usize {
        self.y as usize
    }
}

impl<T: AddAssign> AddAssign for Vector<T> {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl<T: Add<Output = T>> Add for Vector<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Vector<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

pub struct Scalar<T>(pub T);

impl<T: Float> Div<Vector<T>> for Scalar<T> {
    type Output = Vector<T>;

    fn div(self, other: Vector<T>) -> Vector<T> {
        Vector {
            x: self.0 / other.x,
            y: self.0 / other.y,
        }
    }
}

impl<T: Neg<Output = T>> Neg for Vector<T> {
    type Output = Self;

    fn neg(self) -> Self {
        Vector {
            x: -self.x,
            y: -self.y,
        }
    }
}

pub trait ConvertTo<T> {
    fn convert(self) -> T;
}

impl<T, U> ConvertTo<Vector<U>> for Vector<T>
where
    U: From<T>,
{
    fn convert(self) -> Vector<U> {
        Vector {
            x: self.x.into(),
            y: self.y.into(),
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
