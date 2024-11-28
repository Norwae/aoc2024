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
        if length > 0 {
            for (next, _) in self.storage.iter_mut() {
                if *next == 0 {
                    *next = length;
                    return;
                }
            }

            unreachable!()
        }
    }
    fn cursor(&mut self, initial_offset: usize) -> RingListCursor<T> {
        assert!(initial_offset < self.storage.len());
        RingListCursor { list: self, offset: initial_offset }
    }
}

impl <T> RingListCursor<'_, T> {
    fn advance(&mut self, n: usize) {
        for _ in 0..n{
            self.offset = self.list.storage[self.offset].0
        }
    }

    fn current(&self) -> &T {
        &self.list.storage[self.offset].1
    }

    fn insert(&mut self, value: T){
        let next = self.list.storage[self.offset].0;
        self.list.storage.push((next, value));
        let len = self.list.storage.len() - 1;
        self.list.storage[self.offset].0 = len;
        self.offset = len;
    }
}

fn p1(stride: &mut usize) -> u32 {
    let stride = *stride;
    let mut list = RingList::default();
    list.push(0);
    let mut cursor = list.cursor(0);
    for i in 1..=50_000_000 {
        cursor.advance(stride);
        cursor.insert(i);
    }
    cursor.advance(1);

    *cursor.current()
}

parsed_day!(|str|str.trim().parse::<usize>(), p1);