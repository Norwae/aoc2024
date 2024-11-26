use std::mem::swap;
use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct Vec2D<T> {
    storage: Vec<T>,
    row_length: usize
}

impl <T> From<Vec<T>> for Vec2D<T> {
    fn from(value: Vec<T>) -> Self {
        let row_length = value.len();
        assert_ne!(0, row_length);
        Self {
            storage: value,
            row_length
        }
    }
}

impl <T> Index<usize> for Vec2D<T> {
    type Output = [T];

    fn index(&self, index: usize) -> &Self::Output {
        let start = index * self.row_length;
        let end = start + self.row_length;
        &self.storage[start..end]
    }
}

impl <T> IndexMut<usize> for Vec2D<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let start = index * self.row_length;
        let end = start + self.row_length;
        &mut self.storage[start..end]
    }
}

impl <T> Vec2D<T> {
    pub fn new(row_length: usize) -> Self {
        assert_ne!(row_length, 0);
        Self { storage: Vec::new(), row_length }
    }

    pub fn append_row_raw(&mut self, mut source: Vec<T>) {
        assert_eq!(self.row_length, source.len());

        self.storage.append(&mut source)
    }

    pub fn pop_row(&mut self) -> Vec<T> {
        let mut buffer = Vec::new();
        if self.rows() == 1 {
            swap(&mut buffer, &mut self.storage)
        } else {
            for _ in 0..self.row_length {
                buffer.push(self.storage.pop().expect("nonempty"))
            }
            buffer.reverse()
        }
        buffer
    }

    pub fn row_length(&self) -> usize {
        self.row_length
    }

    pub fn rows(&self) -> usize {
        self.storage.len() / self.row_length
    }
}

impl <T: Clone> Vec2D<T> {
    pub fn extend_with_row(&mut self, row: impl AsRef<[T]>) {
        let row = row.as_ref();
        assert_eq!(self.row_length, row.len());
        self.storage.extend_from_slice(row);
    }
}