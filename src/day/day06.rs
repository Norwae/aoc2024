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
}), part1, |_,_|"<see before>");

fn find_repeat(previous: &Vec2D<u8>, next: &Vec<u8>) -> Option<usize> {
    for i in 0..previous.rows() {
        if &previous[i] == next.as_slice() {
            return Some(i);
        }
    }

    None
}

fn redistribute(buffer: &mut [u8]) {
    let (max_index, to_distribute) = buffer.iter().enumerate().rev().max_by_key(|(_, content)| **content).unwrap();
    let to_distribute = *to_distribute;
    buffer[max_index] = 0;
    let n = buffer.len();
    let base = to_distribute / n as u8;
    let mut bonus = to_distribute as i32 % n as i32;
    for i in 0..n {
        buffer[(max_index + 1 + i) % n] += base + if bonus > 0 { 1 } else { 0 };
        bonus -= 1;
    }
}

fn part1<T>(input: &mut Day6Data, _: &mut T) -> String {
    let input = &mut input.evolution;
    let mut buffer = input.pop_row();
    let mut repeat = find_repeat(input, &buffer);
    while repeat.is_none() {
        input.extend_with_row(&buffer);
        redistribute(&mut buffer);
        repeat = find_repeat(input, &buffer)
    }

    format!("At {}, length {}", input.rows(), input.rows() - repeat.unwrap())
}
