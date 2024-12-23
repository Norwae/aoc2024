use std::fmt::{Display, Formatter};
use crate::parse_helpers::parse_unsigned_nr_bytes;
use crate::*;
use nom::character::complete::line_ending;
use nom::sequence::terminated;
use std::ops::AddAssign;

fn step(mut next: usize) -> usize {
    next = ((next << 6) ^ next) & 0x00ff_ffff;
    next = ((next >> 5) ^ next) & 0x00ff_ffff;
    ((next << 11) ^ next) & 0x00ff_ffff
}

#[derive(Debug, Default)]
struct DeltaKey(u64);

impl DeltaKey {
    const SLICE_LENGTH: usize = 1 << 20;
    fn slice_key(&self) -> usize {
        let bytes = self.0.to_le_bytes();
        let d0 = (9 + bytes[1] - bytes[0]) as usize;
        let d1 = (9 + bytes[2] - bytes[1]) as usize;
        let d2 = (9 + bytes[3] - bytes[2]) as usize;
        let d3 = (9 + bytes[4] - bytes[3]) as usize;


        (d0 << 15) | (d1 << 10) | (d2 << 5) | d3
    }
}

impl AddAssign<u8> for DeltaKey {
    fn add_assign(&mut self, price: u8) {
        self.0 = (self.0 << 8) | price as u64;
    }
}

struct Solution {
    sum: usize,
    best_profit: u16,
    memory: Vec<u16>
}

impl Default for Solution {
    fn default() -> Self {
        Self { sum: 0, best_profit: 0, memory: vec![0u16; DeltaKey::SLICE_LENGTH] }
    }
}

impl Display for Solution {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Sum = {}, Max profit = {}", self.sum, self.best_profit))
    }
}

fn price(n: usize) -> u8 {
    (n % 10) as u8
}

fn monkey_nr_sum(data: &mut Solution, mut next: usize) {
    let mut seen = vec![false; DeltaKey::SLICE_LENGTH];
    let mut dk = DeltaKey::default();
    dk += price(next);
    next = step(next);
    dk += price(next);
    next = step(next);
    dk += price(next);

    for _ in 2..2000 {
        next = step(next);
        let price = price(next);
        dk += price;
        let key = dk.slice_key();
        if !seen[key] {
            seen[key] = true;
            let next = data.memory[key] + price as u16;
            data.memory[key] = next;
            if next > data.best_profit {
                data.best_profit = next;
            }
        }
    }

    data.sum += next;
}

streaming_day!(
    terminated(parse_unsigned_nr_bytes::<usize>, line_ending),
    monkey_nr_sum
);
