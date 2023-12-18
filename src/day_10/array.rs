use std::fmt::Display;

use super::vector::Vector;

pub type Array<T> = Vec<Vec<T>>;

pub trait ArrayManipulator<T: Clone + Display> {
    fn data(&self) -> &Array<T>;
    fn data_mut(&mut self) -> &mut Array<T>;

    fn get(&self, at: &Vector) -> Option<T> {
        self.data()
            .get(at.y as usize)
            .and_then(|line| line.get(at.x as usize).cloned())
    }

    fn set(&mut self, at: &Vector, value: T) {
        self.data_mut()[at.y as usize][at.x as usize] = value;
    }

    fn width(&self) -> usize {
        self.data().first().map(|line| line.len()).unwrap_or(0)
    }

    fn height(&self) -> usize {
        self.data().len()
    }

    fn dimensions(&self) -> Vector {
        Vector {
            x: self.width() as i32,
            y: self.height() as i32,
        }
    }

    fn print(&self) {
        self.data().iter().for_each(|line| {
            line.iter().for_each(|tile| print!("{}", tile));
            println!();
        });
    }
}
