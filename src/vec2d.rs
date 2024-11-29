use std::mem::swap;
use std::ops::{Add, Index, IndexMut};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum CompassDirection  {
    NORTH, EAST, SOUTH, WEST
}

impl CompassDirection {
    pub const ALL: [CompassDirection; 4] = [CompassDirection::NORTH, CompassDirection::EAST, CompassDirection::SOUTH, CompassDirection::WEST];
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Index2D {
    pub row: usize,
    pub column: usize,
}

impl Index2D {
    const IMPLAUSIBLE: Index2D = Index2D { row: usize::MAX, column: usize::MAX };
    pub fn plausible(self) -> bool {
        self.row != Self::IMPLAUSIBLE.row && self.column != Self::IMPLAUSIBLE.column
    }
}

impl Add<CompassDirection> for Index2D {
    type Output = Index2D;

    fn add(mut self, rhs: CompassDirection) -> Self::Output {
        if !self.plausible() {
            Self::IMPLAUSIBLE
        } else {
            match rhs {
                CompassDirection::NORTH => self.row = self.row.wrapping_sub(1),
                CompassDirection::EAST => self.column += 1,
                CompassDirection::SOUTH => self.row += 1,
                CompassDirection::WEST => self.column = self.column.wrapping_sub(1),
            }
            self
        }
    }
}

#[derive(Debug)]
pub struct Vec2D<T> {
    storage: Vec<T>,
    row_length: usize,
}

impl<T> From<Vec<T>> for Vec2D<T> {
    fn from(value: Vec<T>) -> Self {
        let row_length = value.len();
        assert_ne!(0, row_length);
        Self {
            storage: value,
            row_length,
        }
    }
}

impl<T> Index<usize> for Vec2D<T> {
    type Output = [T];

    fn index(&self, index: usize) -> &Self::Output {
        let start = index * self.row_length;
        let end = start + self.row_length;
        &self.storage[start..end]
    }
}

impl<T> IndexMut<usize> for Vec2D<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let start = index * self.row_length;
        let end = start + self.row_length;
        &mut self.storage[start..end]
    }
}

impl<T> Index<Index2D> for Vec2D<T> {
    type Output = T;

    fn index(&self, index: Index2D) -> &Self::Output {
        &self[index.row][index.column]
    }
}

impl<T> IndexMut<Index2D> for Vec2D<T> {
    fn index_mut(&mut self, index: Index2D) -> &mut Self::Output {
        &mut self[index.row][index.column]
    }
}

impl<T> Vec2D<T> {
    pub fn new(row_length: usize) -> Self {
        assert_ne!(row_length, 0);
        Self { storage: Vec::new(), row_length }
    }

    pub fn validate_index(&self, idx: Index2D) -> bool {
        idx.column < self.row_length && idx.row < self.storage.len() / self.row_length
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

    pub fn find<T2: PartialEq<T>>(&self, value: &T2) -> Option<Index2D> {
        if let Some((linear_idx, _)) = self.storage.iter().enumerate().filter(|(_, it)|value.eq(it)).next() {
            Some(Index2D { row: linear_idx / self.row_length, column: linear_idx % self.row_length() })
        } else {
            None
        }
    }
}


impl<T: Clone> Vec2D<T> {
    pub fn extend_with_row(&mut self, row: impl AsRef<[T]>) {
        let row = row.as_ref();
        assert_eq!(self.row_length, row.len());
        self.storage.extend_from_slice(row);
    }


    pub fn filled(prototype: T, rows: usize, columns: usize) -> Self {
        Self { storage: vec![prototype; rows * columns], row_length: rows }
    }
}