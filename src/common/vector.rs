use super::array::Coordinate;

#[derive(Debug, Clone, Default, PartialEq, Hash, Eq)]
pub struct Vector {
    pub x: i32,
    pub y: i32,
}

impl Vector {
    pub fn add(&self, other: &Vector) -> Self {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn neg(&self) -> Self {
        Vector {
            x: -self.x,
            y: -self.y,
        }
    }

    pub fn manhattan_distance(&self, other: &Vector) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
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
