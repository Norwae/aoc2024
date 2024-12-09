use std::array;
use std::fmt::{Debug, Formatter};
use std::mem::MaybeUninit;
use std::ops::{Add, AddAssign, Sub, SubAssign};

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
        if index >= N {
            panic!("Out of bounds!")
        }
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

pub struct ArrayBag<T, const N: usize> {
    storage: [MaybeUninit<T>; N],
    empty_slot: usize,
}

impl<T: Debug, const N: usize> Debug for ArrayBag<T, N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self.as_ref()))
    }
}

impl<T, const N: usize> Default for ArrayBag<T, N> {
    fn default() -> Self {
        Self { storage: [const { MaybeUninit::uninit() }; N], empty_slot: 0 }
    }
}

impl<T, const N: usize> AsRef<[T]> for ArrayBag<T, N> {
    fn as_ref(&self) -> &[T] {
        unsafe {
            // safe - we have read access, so empty_slot cannot change, and we ensure
            // no uninit exists past that threshold
            MaybeUninit::slice_assume_init_ref(&self.storage[..self.empty_slot])
        }
    }
}

impl<T, const N: usize> ArrayBag<T, N> {
    pub fn is_empty(&self) -> bool {
        self.empty_slot == 0
    }

    pub fn iter(&self) -> impl Iterator<Item=&T> {
        self.storage[0..self.empty_slot].iter().map(|i| {
            unsafe {
                // safe - we have read access, so empty_slot cannot change. Consequently, no
                // chance to hit the uninitialized tail end of empty_slot and further
                i.assume_init_ref()
            }
        })
    }


    pub fn insert(&mut self, elem: T) {
        if self.empty_slot == N {
            panic!("Overflowing limit")
        }
        let slot = self.empty_slot;
        self.storage[slot].write(elem);
        self.empty_slot += 1;
    }
}

impl<T: Eq, const N: usize> ArrayBag<T, N> {
    pub fn remove(&mut self, value: &T) {
        if let Some(idx) = self.as_ref().iter().position(|it| it == value) {
            self.storage.swap(idx, self.empty_slot - 1);
            self.empty_slot -= 1;
            unsafe {
                // safe - if drop were to panic, we already decremented the count, essentially
                // only leaking the instance
                self.storage[self.empty_slot].assume_init_drop()
            }
        }
    }

    pub fn insert_if_absent(&mut self, elem: T) -> bool {
        if !self.as_ref().contains(&elem) {
            self.insert(elem);
            true
        } else {
            false
        }
    }
}

impl<T: Clone, const N: usize> Clone for ArrayBag<T, N> {
    fn clone(&self) -> Self {
        let mut container = Self::default();
        let slice = self.as_ref();
        for slot in 0..self.empty_slot {
            container.storage[slot].write(slice[slot].clone());
            container.empty_slot += 1;
        }

        container
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
pub struct Location2D {
    pub row: i64,
    pub column: i64,
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

impl Location2D {
    pub const ZERO: Index2D = Index2D { row: 0, column: 0 };

    pub fn move_by(mut self, steps: usize, direction: CompassDirection) -> Location2D {
        let steps = steps as i64;
        match direction {
            CompassDirection::NORTH => self.row -= steps ,
            CompassDirection::EAST => self.column += steps,
            CompassDirection::SOUTH => self.row += steps,
            CompassDirection::WEST => self.column -= steps,
        }

        self
    }
    pub fn manhattan_distance(&self, other: Location2D) -> usize {
        let d0 = (self.row - other.row).abs();
        let d1 = (self.column - other.column).abs();

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

    fn add(self, rhs: CompassDirection) -> Self::Output {
        self.move_by(1, rhs)
    }
}

impl Sub<CompassDirection> for Index2D {
    type Output = Index2D;

    fn sub(self, rhs: CompassDirection) -> Self::Output {
        self.move_by(1, rhs.turn_right().turn_right())
    }
}

impl AddAssign<CompassDirection> for Index2D {
    fn add_assign(&mut self, rhs: CompassDirection) {
        *self = self.move_by(1, rhs)
    }
}

impl SubAssign<CompassDirection> for Index2D {
    fn sub_assign(&mut self, rhs: CompassDirection) {
        *self = *self - rhs
    }
}



impl Add for Location2D {
    type Output = Location2D;

    fn add(self, rhs: Location2D) -> Self::Output {
        Self {
            column: self.column + rhs.column,
            row: self.row + rhs.row,
        }
    }
}

impl AddAssign for Location2D {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl Add<CompassDirection> for Location2D {
    type Output = Location2D;

    fn add(self, rhs: CompassDirection) -> Self::Output {
        self.move_by(1, rhs)
    }
}

impl Sub<CompassDirection> for Location2D {
    type Output = Location2D;

    fn sub(self, rhs: CompassDirection) -> Self::Output {
        self.move_by(1, rhs.turn_right().turn_right())
    }
}

impl AddAssign<CompassDirection> for Location2D {
    fn add_assign(&mut self, rhs: CompassDirection) {
        *self = self.move_by(1, rhs)
    }
}

impl SubAssign<CompassDirection> for Location2D {
    fn sub_assign(&mut self, rhs: CompassDirection) {
        *self = *self - rhs
    }
}

impl Sub for Location2D {
    type Output = Location2D;

    fn sub(self, rhs: Self) -> Self::Output {
        Location2D { row: self.row - rhs.row, column: self.column - rhs.column }
    }
}

impl Into<Index2D> for Location2D {
    fn into(self) -> Index2D {
        if self.row >= 0 && self.column >= 0 {
            Index2D { row: self.row as usize, column: self.column as usize}
        } else {
            Index2D::IMPLAUSIBLE
        }
    }
}

impl Into<Location2D> for Index2D {
    fn into(self) -> Location2D {
        if !self.plausible() {
            panic!("Implausible cannot be resolved anymore")
        }

        Location2D { row: self.row as i64, column: self.column as i64 }
    }
}