use std::ops::{RangeBounds, RangeInclusive};
use nom::character::complete::{line_ending, space1};
use nom::multi::separated_list1;
use crate::*;
use crate::day::nom_parsed;
use crate::parse_helpers::parse_unsigned_nr;

parsed_day!(
    nom_parsed(
        separated_list1(line_ending, separated_list1(space1, parse_unsigned_nr::<i32>))
    ),
    count_safe_lines,
    count_safe_lines_with_tolerance
);

const ACCEPTABLE_DELTA: RangeInclusive<i32> = 1..=3;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    DESCENDING,
    ASCENDING,
}

impl Direction {
    fn from_difference(a: i32, b: i32) -> Option<Self> {
        if b < a {
            Some(Direction::DESCENDING)
        } else if b > a {
            Some(Direction::ASCENDING)
        } else {
            None
        }
    }
}

fn line_is_safe_simple(line: &[i32]) -> bool {
    if line.len() < 2 {
        return true;
    }
    let direction = Direction::from_difference(line[0], line[1]);
    if direction.is_none() {
        return false;
    }
    let direction = direction.unwrap();
    for s in line.windows(2) {
        let l = s[0];
        let r = s[1];
        let diff = if direction == Direction::DESCENDING {
            l - r
        } else {
            r - l
        };

        if !ACCEPTABLE_DELTA.contains(&diff) {
            return false;
        }
    }
    true
}

fn line_is_safe_with_tolerance(line: &Vec<i32>, skip_buffer: &mut [i32]) -> bool {
    let mut skip_buffer = &mut skip_buffer[0..line.len() - 1];
    for i in 0..=line.len() - 1 {
        (&mut skip_buffer[0..i]).copy_from_slice(&line[0..i]);
        (&mut skip_buffer[i..]).copy_from_slice(&line[i+1..]);

        if line_is_safe_simple(skip_buffer) {
            return true
        }
    }
    false
}


fn count_safe_lines(input: &mut Vec<Vec<i32>>) -> usize {
    input.iter().filter(|l| line_is_safe_simple(l)).count()
}

fn count_safe_lines_with_tolerance(input: Vec<Vec<i32>>) -> usize {
    let mut buffer = [0i32; 128];
    input.into_iter().filter(|l| line_is_safe_with_tolerance(l, &mut buffer)).count()
}