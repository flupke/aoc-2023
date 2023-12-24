use std::ops::{Add, Index, IndexMut};
use std::{fmt::Display, usize};

use itertools::Itertools;

use super::vector::Vector;

#[derive(Clone, Debug, PartialEq)]
pub struct Array<T> {
    data: Vec<Vec<T>>,
}

#[allow(dead_code)]
impl<T: Display + Clone> Array<T> {
    pub fn new(data: Vec<Vec<T>>) -> Self {
        Self { data }
    }

    pub fn get(&self, at: &Vector) -> Option<T> {
        self.data
            .get(at.y as usize)
            .and_then(|line| line.get(at.x as usize).cloned())
    }

    pub fn set(&mut self, at: &Vector, value: T) {
        self.data[at.y as usize][at.x as usize] = value;
    }

    pub fn width(&self) -> usize {
        self.data.first().map(|line| line.len()).unwrap_or(0)
    }

    pub fn height(&self) -> usize {
        self.data.len()
    }

    pub fn dimensions(&self) -> Vector {
        Vector {
            x: self.width() as i32,
            y: self.height() as i32,
        }
    }

    pub fn print(&self) {
        print!("{}", self.format());
    }

    pub fn format(&self) -> String {
        self.data
            .iter()
            .map(|line| line.iter().map(|item| format!("{}", item)).join(""))
            .join("\n")
    }
}

#[allow(dead_code)]
impl<T: Add<Output = T> + Clone> Array<T> {
    pub fn add(&self, other: &Self) -> Self {
        Self {
            data: self
                .data
                .iter()
                .zip(other.data.iter())
                .map(|(line_1, line_2)| {
                    line_1
                        .iter()
                        .zip(line_2.iter())
                        .map(|(item_1, item_2)| item_1.clone() + item_2.clone())
                        .collect()
                })
                .collect(),
        }
    }

    pub fn shift(&self, value: &T) -> Self {
        Self {
            data: self
                .data
                .iter()
                .map(|line| {
                    line.iter()
                        .map(|item| item.clone() + value.clone())
                        .collect()
                })
                .collect(),
        }
    }
}

impl<T> FromIterator<Vec<T>> for Array<T> {
    fn from_iter<I: IntoIterator<Item = Vec<T>>>(iter: I) -> Self {
        Self {
            data: iter.into_iter().collect(),
        }
    }
}

impl<T> Index<usize> for Array<T> {
    type Output = Vec<T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T> IndexMut<usize> for Array<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}
