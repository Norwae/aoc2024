use std::fmt::{Debug, Formatter};
use std::mem::{ManuallyDrop, MaybeUninit};
use std::ops::{Add, AddAssign, Index, IndexMut, Sub, SubAssign};

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
        Self {
            storage: [const { None }; N],
            mappings: 0,
        }
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

    pub fn values_iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.iter_mut().map(|(_, v)| v)
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (usize, &mut T)> {
        self.storage
            .iter_mut()
            .enumerate()
            .filter_map(|(n, stored)| match stored {
                None => None,
                Some(v) => Some((n, v)),
            })
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= N {
            panic!("Out of bounds!")
        }
        self.storage[index].as_ref()
    }

    pub fn iter(&self) -> impl Iterator<Item = (usize, &T)> {
        self.storage
            .iter()
            .enumerate()
            .filter_map(|(n, stored)| match stored {
                None => None,
                Some(v) => Some((n, v)),
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
        Self {
            storage: [const { MaybeUninit::uninit() }; N],
            empty_slot: 0,
        }
    }
}

impl<T, const N: usize> AsRef<[T]> for ArrayBag<T, N> {
    fn as_ref(&self) -> &[T] {
        unsafe {
            // safe - we have read access, so empty_slot cannot change, and we ensure
            // no uninit exists past that threshold. Thus, the contract of slice_assume_init_ref is
            // fulfilled
            MaybeUninit::slice_assume_init_ref(&self.storage[..self.empty_slot])
        }
    }
}

impl<T, const N: usize> ArrayBag<T, N> {
    fn clear(&mut self) {
        while self.empty_slot > 0 {
            self.empty_slot -= 1;
            unsafe {
                // similar reasoning to remove, we first decrement
                // to avoid a panicky destructor making us point to bad memory,
                // then destroy the elements one by one. A smart drop like Vec might
                // be preferable but this is safe and reasonably efficient
                self.storage[self.empty_slot].assume_init_drop();
            }
        }
    }
    pub fn is_empty(&self) -> bool {
        self.empty_slot == 0
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
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
    
    pub fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for elem in iter {
            self.insert(elem);
        }
    }
}

impl<T, const N: usize> Drop for ArrayBag<T, N> {
    fn drop(&mut self) {
        self.clear();
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


pub struct ArrayBagIter<T, const N: usize> {
    head: usize,
    bag: ManuallyDrop<ArrayBag<T, N>>
}


/**
IntoIterator view of an ArrayBag - will violate the initialization
rules of a living ArrayBag (0..empty_slot always being valid) by instead
retaining validity between (head..empty_slot) only, reading entries from the start
*/
impl <T, const N: usize> Iterator for ArrayBagIter<T, N> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.head >= self.bag.empty_slot {
            None
        } else {
            // safety - this takes ownership of the initialized value, freeing us
            // from the responsibility of dropping it later.
            let read_slot = self.head;
            let value = unsafe {
                self.head += 1;
                self.bag.storage[read_slot].assume_init_read()
            };

            Some(value)
        }
    }
}

impl <T, const N: usize> Drop for ArrayBagIter<T, N> {
    fn drop(&mut self) {
        while let Some(_) = self.next() {
            // simple inefficient implementation - just read and drop the remaining elements
        }
    }
}


impl <T, const N: usize> IntoIterator for ArrayBag<T, N> {
    type Item = T;
    type IntoIter = ArrayBagIter<T, N>;

    fn into_iter(self) -> Self::IntoIter {
        ArrayBagIter {
            head: 0,
            bag: ManuallyDrop::new(self),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Default, Hash)]
pub enum CompassDirection {
    #[default]
    NORTH,
    EAST,
    SOUTH,
    WEST,
}

impl CompassDirection {
    pub const ALL: [CompassDirection; 4] = [
        CompassDirection::NORTH,
        CompassDirection::EAST,
        CompassDirection::SOUTH,
        CompassDirection::WEST,
    ];

    pub fn turn_right(self) -> Self {
        match self {
            CompassDirection::NORTH => CompassDirection::EAST,
            CompassDirection::EAST => CompassDirection::SOUTH,
            CompassDirection::SOUTH => CompassDirection::WEST,
            CompassDirection::WEST => CompassDirection::NORTH,
        }
    }


    pub fn turn_left(self) -> Self {
        match self {
            CompassDirection::NORTH => CompassDirection::WEST,
            CompassDirection::EAST => CompassDirection::NORTH,
            CompassDirection::SOUTH => CompassDirection::EAST,
            CompassDirection::WEST => CompassDirection::SOUTH,
        }
    }
}
/**
2D Indices, unconstrained for arbitrary movement - need to be converted to Index2D for usage in
Vec2D access
*/
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Hash)]
pub struct Location2D {
    pub row: i64,
    pub column: i64,
}

/** 2D indices, constrained to potentially valid ones - so no negative indices even possible */
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Hash)]
pub struct Index2D {
    pub row: usize,
    pub column: usize,
}


impl Index2D {
    pub const IMPLAUSIBLE: Index2D = Index2D {
        row: usize::MAX,
        column: usize::MAX,
    };
    pub const ZERO: Index2D = Index2D {
        row: 0,
        column: 0,
    };

    pub fn plausible(self) -> bool {
        self.row != Self::IMPLAUSIBLE.row && self.column != Self::IMPLAUSIBLE.column
    }

    pub fn manhattan_distance(self, other: Index2D) -> usize {
        let x = self.column as isize - other.column as isize;
        let y = self.row as isize - other.row as isize;
        (x.abs() + y.abs()) as usize
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
                _ => self = Self::IMPLAUSIBLE,
            }
        }

        self
    }
}

impl Location2D {
    pub fn move_by(mut self, steps: usize, direction: CompassDirection) -> Location2D {
        let steps = steps as i64;
        match direction {
            CompassDirection::NORTH => self.row -= steps,
            CompassDirection::EAST => self.column += steps,
            CompassDirection::SOUTH => self.row += steps,
            CompassDirection::WEST => self.column -= steps,
        }

        self
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
        Location2D {
            row: self.row - rhs.row,
            column: self.column - rhs.column,
        }
    }
}

impl Into<Index2D> for Location2D {
    fn into(self) -> Index2D {
        if self.row >= 0 && self.column >= 0 {
            Index2D {
                row: self.row as usize,
                column: self.column as usize,
            }
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

        Location2D {
            row: self.row as i64,
            column: self.column as i64,
        }
    }
}

#[derive(Debug, Clone)]
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
    pub fn new_from_flat(storage: Vec<T>, row_length: usize) -> Self {
        assert_ne!(row_length, 0);
        Self {
            storage,
            row_length,
        }
    }

    pub fn validate_index(&self, idx: Index2D) -> bool {
        idx.column < self.row_length && idx.row < self.storage.len() / self.row_length
    }

    pub fn row_length(&self) -> usize {
        self.row_length
    }

    pub fn rows(&self) -> usize {
        self.storage.len() / self.row_length
    }

    pub fn as_slice(&self) -> &[T] {
        self.storage.as_slice()
    }

    pub fn len(&self) -> usize {
        self.storage.len()
    }

    pub fn extend_from(&mut self, other: Vec<T>) {
        assert_eq!(self.row_length, other.len());
        self.storage.extend(other);
    }

    pub fn indices(&self) -> impl Iterator<Item = Index2D> {
        let rows = self.rows();
        let columns = self.row_length();

        (0usize..rows)
            .flat_map(move |row| (0usize..columns).map(move |column| Index2D { row, column }))
            .into_iter()
    }
}

impl<T: Clone> Vec2D<T> {
    pub fn filled(default: T, rows: usize, columns: usize) -> Self {
        let storage = vec![default; rows * columns];
        Self::new_from_flat(storage, columns)
    }
}

pub struct Slice2DVisor<'a> {
    bytes: &'a [u8],
    newline_at: usize,
}

static OUTSIDE: u8 = b'!';
impl<'a> Slice2DVisor<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        let newline_at = bytes.iter().position(|it| *it == b'\n').unwrap();

        Self { bytes, newline_at }
    }

    pub fn columns(&self) -> usize {
        self.newline_at
    }

    pub fn rows(&self) -> usize {
        self.bytes.len() / (self.newline_at + 1)
    }

    pub fn at(&self, row: i64, column: i64) -> u8 {
        self[Location2D { row, column }]
    }
}

impl Index<Index2D> for Slice2DVisor<'_> {
    type Output = u8;

    fn index(&self, Index2D { row, column }: Index2D) -> &Self::Output {
        if column >= self.columns() || row >= self.rows() {
            &OUTSIDE
        } else {
            let offset = (self.newline_at + 1) * row as usize + column as usize;
            &self.bytes[offset]
        }
    }
}
impl Index<Location2D> for Slice2DVisor<'_> {
    type Output = u8;

    fn index(&self, Location2D { row, column }: Location2D) -> &Self::Output {
        static OUTSIDE: u8 = b'!';
        if row < 0 || column < 0 || column as usize >= self.columns() || row as usize >= self.rows()
        {
            &OUTSIDE
        } else {
            let offset = (self.newline_at + 1) * row as usize + column as usize;
            &self.bytes[offset]
        }
    }
}
