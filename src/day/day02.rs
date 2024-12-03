use std::mem::swap;
use std::ops::{Index, RangeInclusive};
use nom::character::complete::{line_ending, space1};
use nom::combinator::map;
use nom::multi::separated_list1;
use crate::*;
use crate::day::nom_parsed;
use crate::parse_helpers::parse_unsigned_nr;

struct Day2 {
    candidates: Vec<Vec<i32>>,
    clean: usize,
}

parsed_day!(
    nom_parsed(
        map(separated_list1(line_ending, separated_list1(space1, parse_unsigned_nr::<i32>)), |candidates|Day2{candidates, clean: 0})
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

    fn validate_step(self, l: i32, r: i32) -> bool {
        let diff = if self == Direction::ASCENDING { r - l } else { l - r };
        ACCEPTABLE_DELTA.contains(&diff)
    }
}

fn line_is_safe_simple(line: &(impl Index<usize, Output=i32> + ?Sized), length: usize) -> bool {
    let direction = Direction::from_difference(line[0], line[1]);
    if direction.is_none() {
        return false;
    }
    let direction = direction.unwrap();
    for i in 0..length - 1 {
        let l = line[i];
        let r = line[i + 1];
        if !direction.validate_step(l, r) {
            return false;
        }
    }
    true
}

struct SkipSlice<'a>(&'a [i32], usize);
impl <'a> Index<usize> for SkipSlice<'a> {
    type Output = i32;

    fn index(&self, index: usize) -> &'a Self::Output {
        if index < self.1 {
            &self.0[index]
        } else {
            &self.0[index + 1]
        }
    }
}

fn line_is_safe_with_tolerance(line: &Vec<i32>) -> bool {
    let reduced_line_length = line.len() - 1;
    for i in 0..=reduced_line_length {
        if line_is_safe_simple(&SkipSlice(line, i), reduced_line_length) {
            return true;
        }
    }
    false
}


fn count_safe_lines(input: &mut Day2) -> usize {
    let Day2 { candidates, clean } = input;
    let mut tmp = Vec::new();
    let mut count = 0;
    swap(&mut tmp, candidates);

    for line in tmp.into_iter() {
        if line_is_safe_simple(&line, line.len()) {
            count += 1;
        } else {
            candidates.push(line)
        }
    }
    *clean = count;

    count
}

fn count_safe_lines_with_tolerance(input: Day2) -> usize {
    input.clean +
        input.candidates.into_iter().filter(|l| line_is_safe_with_tolerance(l)).count()
}