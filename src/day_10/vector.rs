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
}
