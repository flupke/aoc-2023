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

    pub fn rows(&self) -> impl Iterator<Item = &Vec<T>> {
        self.data.iter()
    }

    pub fn columns(&self) -> Vec<Vec<T>> {
        (0..self.width())
            .map(move |x| self.rows().map(move |line| line[x].clone()).collect())
            .collect()
    }

    pub fn print(&self) {
        println!("{}", self.format());
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

impl<T: Default + Clone> Array<T> {
    pub fn empty(width: usize, height: usize) -> Self {
        Self {
            data: vec![vec![T::default(); width]; height],
        }
    }
}

pub struct ArrayIterator<'a, T> {
    array: &'a Array<T>,
    y: usize,
    x: usize,
}

impl<'a, T: Display + Clone> Iterator for ArrayIterator<'a, T> {
    type Item = (Vector, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        self.array
            .data
            .get(self.y)
            .and_then(|line| line.get(self.x))
            .and_then(|item| {
                let result = Some((
                    Vector {
                        x: self.x as i32,
                        y: self.y as i32,
                    },
                    item,
                ));
                self.x += 1;
                if self.x == self.array.width() {
                    self.x = 0;
                    self.y += 1;
                }
                result
            })
    }
}

impl<'a, T: Display + Clone> IntoIterator for &'a Array<T> {
    type Item = (Vector, &'a T);
    type IntoIter = ArrayIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        ArrayIterator {
            array: self,
            y: 0,
            x: 0,
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

pub fn parse(input: &str) -> Array<char> {
    input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Array<char>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iteration() {
        let array = Array::new(vec![vec![1, 2, 3], vec![4, 5, 6]]);
        assert_eq!(
            array.into_iter().collect::<Vec<(Vector, &i32)>>(),
            vec![
                (Vector { x: 0, y: 0 }, &1),
                (Vector { x: 1, y: 0 }, &2),
                (Vector { x: 2, y: 0 }, &3),
                (Vector { x: 0, y: 1 }, &4),
                (Vector { x: 1, y: 1 }, &5),
                (Vector { x: 2, y: 1 }, &6)
            ]
        );
    }
}
