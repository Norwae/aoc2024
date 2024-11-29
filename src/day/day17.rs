use crate::*;

#[derive(Debug, Default)]
struct RingList<T> {
    storage: Vec<(usize, T)>,
}
struct RingListCursor<'a, T> {
    list: &'a mut RingList<T>,
    offset: usize,
}

impl<T> RingList<T> {
    fn push(&mut self, value: T) {
        let length = self.storage.len();
        self.storage.push((0, value));
        for (next, _) in self.storage.iter_mut().take(length).rev() {
            if *next == 0 {
                *next = length;
                return;
            }
        }
    }

    fn cursor(&mut self, initial_offset: usize) -> RingListCursor<T> {
        assert!(initial_offset < self.storage.len());
        RingListCursor { list: self, offset: initial_offset }
    }
}

impl<T> RingListCursor<'_, T> {
    pub fn advance(&mut self, n: usize) {
        for _ in 0..n {
            self.offset = self.list.storage[self.offset].0
        }
    }

    pub fn position(&self) -> usize {
        self.offset
    }

    pub fn current(&self) -> &T {
        &self.list.storage[self.offset].1
    }

    pub fn insert(&mut self, value: T) {
        let next = self.list.storage[self.offset].0;
        self.list.storage.push((next, value));
        let len = self.list.storage.len() - 1;
        self.list.storage[self.offset].0 = len;
        self.offset = len;
    }
}

fn p1(stride: &mut u32) -> usize {
    let stride = *stride;
    let mut list = RingList::default();
    list.push(());
    let mut cursor = list.cursor(0);
    for i in 1..=2017 {
        cursor.advance(stride as usize);
        cursor.insert(());
    }
    cursor.advance(1);

    cursor.position()
}

fn p2(stride: u32) -> u32 {
    let mut cursor = 0;
    let mut one_contents = 0;

    for i in 1u32..=50000000 {
        cursor = ((cursor + stride) % i) + 1;
        if cursor == 1 {
            one_contents = i
        }
    }

    one_contents
}

parsed_day!(|str|str.trim().parse::<u32>(), p1, p2);