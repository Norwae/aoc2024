use nom::character::complete::{digit1, space1};
use nom::combinator::{map, map_res};
use nom::multi::separated_list1;
use crate::*;
use crate::day::nom_parsed;
use crate::vec2d::Vec2D;

struct Day6Data {
    evolution: Vec2D<u8>,
}

parsed_day!(nom_parsed(|str|{
    map(separated_list1(
        space1,
        map_res(digit1, |s: &str|s.parse::<u8>())
    ), |input|{
        Day6Data{ evolution: input.into() }
    })(str)
}), part1, |_|"<see before>");

fn find_repeat(previous: &Vec2D<u8>, next: &Vec<u8>) -> Option<usize> {
    for i in 0..previous.rows() {
        if &previous[i] == next.as_slice() {
            return Some(i);
        }
    }

    None
}

fn redistribute(buffer: &mut [u8], from: usize) -> usize {
    let to_distribute = buffer[from];
    buffer[from] = 0;
    let n = buffer.len();
    let base = to_distribute / n as u8;
    let mut bonus = to_distribute as i32 % n as i32;
    let mut next_max = 0;
    let mut next_max_index = from;
    for i in 0..n {
        let idx = (from + 1 + i) % n;
        buffer[idx] += base + if bonus > 0 { 1 } else { 0 };
        bonus -= 1;

        if buffer[idx] > next_max || (buffer[idx] == next_max && idx < next_max_index) {
            next_max = buffer[idx];
            next_max_index = idx;
        }
    }

    next_max_index
}

fn part1(input: &mut Day6Data) -> String {
    let input = &mut input.evolution;
    let mut buffer = input.pop_row();
    let mut repeat = find_repeat(input, &buffer);
    let mut redistribute_index = buffer.iter().enumerate().rev().max_by_key(|(_, value)|**value).unwrap().0;
    while repeat.is_none() {
        input.extend_with_row(&buffer);
        redistribute_index = redistribute(&mut buffer, redistribute_index);
        repeat = find_repeat(input, &buffer)
    }

    format!("At {}, length {}", input.rows(), input.rows() - repeat.unwrap())
}
