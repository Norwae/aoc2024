use crate::*;

pub struct KnotHash {
    values: [u8; 256],
    scratch: [u8; 256],
    cursor: u8,
    skip: u8,
}

const INITIAL_VALUES: [u8; 256] = const {
    let mut storage = [0u8; 256];
    let mut i = 0;
    while i < 256 {
        storage[i] = i as u8;
        i += 1;
    }
    storage
};

impl KnotHash {
    pub const fn new() -> Self {
        Self { values: INITIAL_VALUES, scratch: [0u8; 256], cursor: 0, skip: 0 }
    }

    pub fn update(&mut self, input: &[u8]) {
        for byte in input {
            self.perform_twist(*byte as usize)
        }
    }

    fn perform_twist(&mut self, length: usize) {
        let cursor = self.cursor as usize;
        let available_at_cursor = 256 - cursor;
        let taken_from_back = length.min(available_at_cursor);
        let taken_from_front = length - taken_from_back;
        let (front, back) = self.values.split_at_mut(cursor);
        let back = &mut back[..taken_from_back];
        let front = &mut front[..taken_from_front];
        let scratch = &mut self.scratch[0..length];

        scratch[..taken_from_back].copy_from_slice(back);
        scratch[taken_from_back..].copy_from_slice(front);
        scratch.reverse();
        back.copy_from_slice(&scratch[..taken_from_back]);
        front.copy_from_slice(&scratch[taken_from_back..]);
        self.cursor = self.cursor.wrapping_add(length as u8).wrapping_add(self.skip);
        self.skip = self.skip.wrapping_add(1);
    }

    fn fingerprint(&self) -> u32 {
        self.values[0] as u32 * self.values[1] as u32
    }

    pub fn hash(&self) -> u128 {
        let mut overall_hash = 0u128;
        for chunk_idx in 0..16 {
            overall_hash <<= 8;
            let offset = chunk_idx * 16;
            let chunk = &self.values[offset..offset + 16];
            let chunk_xor = chunk.into_iter().fold(0u8, |xor, next| {
                xor ^ *next
            });

            let widened = chunk_xor as u128;
            overall_hash |= widened;
        }
        overall_hash
    }
}

simple_day!(|input|{
    let input = input.trim();
    let nrs = input.split(",").map(|s|s.parse::<usize>().unwrap());
    let mut list = KnotHash::new();
    for nr in nrs {
        list.perform_twist(nr)
    }

    let part1 = list.fingerprint();
    let mut round_offsets = input.bytes().collect::<Vec<_>>();
    round_offsets.extend_from_slice(&[17, 31, 73, 47, 23]);
    let mut list = KnotHash::new();
    for _ in 0..64 {
        list.update(&round_offsets)
    }
    let part2 = list.hash();

    format!("Part 1: {part1}, Part2: {part2:0>32x}")
});