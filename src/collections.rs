use std::array;
use std::mem::swap;
use std::ops::{Add, AddAssign, Index, IndexMut, Mul};

#[derive(Debug, Clone)]
pub struct IndexMap<T, const N: usize> {
    storage: [Option<T>; N],
    mappings: usize,
}

impl<T, const N: usize> Default for IndexMap<T, N> {
    fn default() -> Self {
        Self {
            mappings: 0,
            storage: [const { None }; N],
        }
    }
}

impl<T, const N: usize> IndexMap<T, N> {
    pub fn new() -> Self {
        Self { storage: [const { None }; N], mappings: 0 }
    }

    pub fn is_empty(&self) -> bool {
        self.mappings == 0
    }

    pub fn remove(&mut self, index: usize) {
        if self.storage[index].is_some() {
            self.storage[index] = None;
            self.mappings -= 1;
        }
    }

    pub fn values_iter_mut(&mut self) -> impl Iterator<Item=&mut T> {
        self.iter_mut().map(|(_, v)| v)
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item=(usize, &mut T)> {
        self.storage.iter_mut().enumerate().filter_map(|(n, stored)| {
            match stored {
                None => None,
                Some(v) => Some((n, v))
            }
        })
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.storage[index].as_ref()
    }

    pub fn iter(&self) -> impl Iterator<Item=(usize, &T)> {
        self.storage.iter().enumerate().filter_map(|(n, stored)| {
            match stored {
                None => None,
                Some(v) => Some((n, v))
            }
        })
    }
}

impl<T: Default, const N: usize> IndexMap<T, N> {
    pub fn get_or_insert_default(&mut self, n: usize) -> &mut T {
        let stored = &mut self.storage[n];
        if stored.is_none() {
            self.mappings += 1;
        }
        stored.get_or_insert_default()
    }
}

#[derive(Debug)]
pub struct ArrayBag<T, const N: usize> {
    storage: [T; N],
    empty_slot: usize,
}

impl<T: Default, const N: usize> Default for ArrayBag<T, N> {
    fn default() -> Self {
        Self { storage: array::from_fn(|_| T::default()), empty_slot: 0 }
    }
}

impl <T, const N: usize> AsRef<[T]> for ArrayBag<T, N> {
    fn as_ref(&self) -> &[T] {
        &self.storage[..self.empty_slot]
    }
}

impl<T, const N: usize> ArrayBag<T, N> {
    pub fn is_empty(&self) -> bool {
        self.empty_slot == 0
    }

    pub fn iter(&self) -> impl Iterator<Item=&T> {
        self.storage[0..self.empty_slot].iter()
    }

    pub fn insert(&mut self, elem: T) {
        let slot = self.empty_slot;
        self.empty_slot += 1;
        self.storage[slot] = elem
    }
}

impl<T: Eq, const N: usize> ArrayBag<T, N> {
    pub fn remove(&mut self, value: &T) {
        if let Some(idx) = self.storage[0..self.empty_slot].iter().position(|it| it == value) {
            self.storage.swap(idx, self.empty_slot - 1);
            self.empty_slot -= 1;
        }
    }
}


#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
pub enum CompassDirection {
    #[default] NORTH,
    EAST,
    SOUTH,
    WEST,
}

impl CompassDirection {
    pub const ALL: [CompassDirection; 4] = [CompassDirection::NORTH, CompassDirection::EAST, CompassDirection::SOUTH, CompassDirection::WEST];

    pub fn turn_right(self) -> Self {
        match self {
            CompassDirection::NORTH => CompassDirection::EAST,
            CompassDirection::EAST => CompassDirection::SOUTH,
            CompassDirection::SOUTH => CompassDirection::WEST,
            CompassDirection::WEST => CompassDirection::NORTH
        }
    }
}

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Hash)]
pub struct Index2D {
    pub row: usize,
    pub column: usize,
}


impl Index2D {
    const IMPLAUSIBLE: Index2D = Index2D { row: usize::MAX, column: usize::MAX };
    pub const ZERO: Index2D = Index2D { row: 0, column: 0 };
    pub fn plausible(self) -> bool {
        self.row != Self::IMPLAUSIBLE.row && self.column != Self::IMPLAUSIBLE.column
    }

    pub fn move_by(mut self, steps: usize, direction: CompassDirection) -> Index2D {
        if !self.plausible() {
            self = Self::IMPLAUSIBLE
        } else {
            match direction {
                CompassDirection::NORTH if self.row >= steps => self.row -= steps,
                CompassDirection::EAST => self.column += steps,
                CompassDirection::SOUTH => self.row += steps,
                CompassDirection::WEST if self.column >= steps => self.column -= steps,
                _ => self = Self::IMPLAUSIBLE
            }
        }

        self
    }
    pub fn manhattan_distance(&self, other: Index2D) -> usize {
        let d0 = (self.row as isize).wrapping_sub(other.row as isize).abs();
        let d1 = (self.column as isize).wrapping_sub(other.column as isize).abs();

        d0.max(d1) as usize
    }
}

impl Add for Index2D {
    type Output = Index2D;

    fn add(self, rhs: Index2D) -> Self::Output {
        Self {
            column: self.column + rhs.column,
            row: self.row + rhs.row,
        }
    }
}

impl AddAssign for Index2D {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl Add<CompassDirection> for Index2D {
    type Output = Index2D;

    fn add(mut self, rhs: CompassDirection) -> Self::Output {
        self.move_by(1, rhs)
    }
}

impl AddAssign<CompassDirection> for Index2D {
    fn add_assign(&mut self, rhs: CompassDirection) {
        *self = self.move_by(1, rhs)
    }
}

// rest of vec2d.rs omitted for now, may be included when called for