use std::ops::Add;
use std::{fmt::Display, usize};

use itertools::Itertools;

use super::vector::Vector;

#[derive(Clone, Debug, PartialEq)]
pub struct Array<T> {
    pub data: Vec<T>,
    pub width: usize,
    pub height: usize,
}

type BoxedIterator<'a, T> = Box<dyn Iterator<Item = T> + 'a>;
pub type NestedIterator<'a, T> = BoxedIterator<'a, BoxedIterator<'a, T>>;

pub trait Coordinate {
    fn x(&self) -> usize;
    fn y(&self) -> usize;
}

pub type ArrayCoordinates = (usize, usize);

impl Coordinate for ArrayCoordinates {
    fn x(&self) -> usize {
        self.0
    }

    fn y(&self) -> usize {
        self.1
    }
}

#[allow(dead_code)]
impl<T: Display + Clone + Default> Array<T> {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            data: vec![T::default(); width * height],
            width,
            height,
        }
    }

    pub fn get<C: Coordinate>(&self, at: C) -> &T {
        &self.data[at.y() * self.width + at.x()]
    }

    pub fn set<C: Coordinate>(&mut self, at: C, value: T) {
        self.data[at.y() * self.width + at.x()] = value;
    }

    pub fn dimensions(&self) -> Vector {
        Vector {
            x: self.width as i32,
            y: self.height as i32,
        }
    }

    pub fn iter_rows(&self) -> NestedIterator<&T> {
        Box::new(self.into_iter())
    }

    pub fn iter_columns(&self) -> NestedIterator<&T> {
        Box::new(ColumnIterator { array: self, x: 0 })
    }

    pub fn iter_coords(&self) -> impl Iterator<Item = ArrayCoordinates> + DoubleEndedIterator + '_ {
        (0..self.height).flat_map(move |y| (0..self.width).map(move |x| (x, y)))
    }

    pub fn iter_vec_coords(&self) -> impl Iterator<Item = Vector> + DoubleEndedIterator + '_ {
        self.iter_coords().map(|(x, y)| Vector {
            x: x as i32,
            y: y as i32,
        })
    }

    pub fn iter_col_coords(
        &self,
    ) -> impl Iterator<Item = ArrayCoordinates> + DoubleEndedIterator + '_ {
        (0..self.width).flat_map(move |x| (0..self.height).map(move |y| (x, y)))
    }

    pub fn iter_vec_col_coords(&self) -> impl Iterator<Item = Vector> + DoubleEndedIterator + '_ {
        self.iter_col_coords().map(|(x, y)| Vector {
            x: x as i32,
            y: y as i32,
        })
    }

    pub fn print(&self) {
        println!("{}", self.format());
    }

    pub fn format(&self) -> String {
        self.iter_rows()
            .map(|line| line.map(|item| format!("{}", item)).join(""))
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
                .map(|(a, b)| a.clone() + b.clone())
                .collect(),
            width: self.width,
            height: self.height,
        }
    }

    pub fn shift(&self, value: &T) -> Self {
        Self {
            data: self
                .data
                .iter()
                .map(|item| item.clone() + value.clone())
                .collect(),
            width: self.width,
            height: self.height,
        }
    }
}

pub struct RowIterator<'a, T> {
    array: &'a Array<T>,
    y: usize,
}

impl<'a, T: Display + Clone> Iterator for RowIterator<'a, T> {
    type Item = Box<dyn Iterator<Item = &'a T> + 'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let start = self.y * self.array.width;
        if start >= self.array.data.len() {
            return None;
        }
        self.y += 1;
        Some(Box::new(
            self.array.data[start..start + self.array.width].iter(),
        ))
    }
}

struct ColumnIterator<'a, T> {
    array: &'a Array<T>,
    x: usize,
}

impl<'a, T: Display + Clone> Iterator for ColumnIterator<'a, T> {
    type Item = Box<dyn Iterator<Item = &'a T> + 'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.x >= self.array.width {
            return None;
        }
        self.x += 1;
        Some(Box::new(ColumnItemsIterator {
            array: self.array,
            x: self.x - 1,
            y: 0,
        }))
    }
}

struct ColumnItemsIterator<'a, T> {
    array: &'a Array<T>,
    x: usize,
    y: usize,
}

impl<'a, T: Display + Clone> Iterator for ColumnItemsIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.y >= self.array.height {
            return None;
        }
        let item = &self.array.data[self.y * self.array.width + self.x];
        self.y += 1;
        Some(item)
    }
}

impl<'a, T: Display + Clone> IntoIterator for &'a Array<T> {
    type Item = Box<dyn Iterator<Item = &'a T> + 'a>;
    type IntoIter = RowIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        RowIterator { array: self, y: 0 }
    }
}

impl<T> FromIterator<Vec<T>> for Array<T> {
    fn from_iter<I: IntoIterator<Item = Vec<T>>>(iter: I) -> Self {
        let mut data = Vec::new();
        let mut width = None;
        let mut height = 0;
        for row in iter {
            if width.is_none() {
                width = Some(row.len());
            } else {
                assert_eq!(width.unwrap(), row.len(), "rows must have the same length");
            }
            data.extend(row);
            height += 1;
        }
        Self {
            data,
            width: width.unwrap_or(0),
            height,
        }
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
    fn test_into_iter() {
        let array = Array::from_iter(vec![vec![1, 2, 3], vec![4, 5, 6]]);
        assert_eq!(
            array
                .into_iter()
                .map(|row| row.cloned().collect::<Vec<_>>())
                .collect::<Vec<_>>(),
            vec![vec![1, 2, 3], vec![4, 5, 6]]
        );
    }

    #[test]
    fn test_iter_rows() {
        let array = Array::from_iter(vec![vec![1, 2, 3], vec![4, 5, 6]]);
        assert_eq!(
            array
                .iter_rows()
                .map(|row| row.cloned().collect::<Vec<_>>())
                .collect::<Vec<_>>(),
            vec![vec![1, 2, 3], vec![4, 5, 6]]
        );
    }

    #[test]
    fn test_iter_columns() {
        let array = Array::from_iter(vec![vec![1, 2, 3], vec![4, 5, 6]]);
        assert_eq!(
            array
                .iter_columns()
                .map(|column| column.cloned().collect::<Vec<_>>())
                .collect::<Vec<_>>(),
            vec![vec![1, 4], vec![2, 5], vec![3, 6]]
        );
    }

    #[test]
    fn test_iter_coords() {
        let array = Array::from_iter(vec![vec![1, 2, 3], vec![4, 5, 6]]);
        assert_eq!(
            array.iter_coords().collect::<Vec<_>>(),
            vec![(0, 0), (1, 0), (2, 0), (0, 1), (1, 1), (2, 1)]
        );
    }

    #[test]
    fn test_iter_col_coords() {
        let array = Array::from_iter(vec![vec![1, 2, 3], vec![4, 5, 6]]);
        assert_eq!(
            array.iter_col_coords().collect::<Vec<_>>(),
            vec![(0, 0), (0, 1), (1, 0), (1, 1), (2, 0), (2, 1)]
        );
    }
}
