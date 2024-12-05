use std::mem::swap;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::combinator::map;
use nom::IResult;
use nom::multi::{many1, separated_list1};
use nom::sequence::{separated_pair, terminated};
use crate::*;
use crate::day::nom_parsed_bytes;
use crate::parse_helpers::parse_unsigned_nr_bytes;

struct Constraint {
    left: u32,
    right: u32,
}

impl Constraint {
    fn validate(&self, pages: &[u32]) -> Option<(usize, usize)> {
        let mut seenRight = usize::MAX;

        for i in 0..pages.len() {
            let p = pages[i];
            if p == self.left {
                if seenRight != usize::MAX {
                    return Some((i, seenRight));
                }
            }

            if p == self.right {
                seenRight = i;
            }
        }

        None
    }
}

fn parse_constraints(input: &[u8]) -> IResult<&[u8], Constraint> {
    map(terminated(separated_pair(parse_unsigned_nr_bytes, tag("|"), parse_unsigned_nr_bytes), line_ending), |(left, right)| Constraint { left, right })(input)
}

fn parse_pagelist(input: &[u8]) -> IResult<&[u8], Vec<u32>> {
    terminated(separated_list1(tag(","), parse_unsigned_nr_bytes), line_ending)(input)
}

fn part1(input: &mut (Vec<Constraint>, Vec<Vec<u32>>)) -> u32 {
    let (constraint, pages) = input;
    let mut tmp = Vec::new();
    swap(&mut tmp, pages);
    let mut sum = 0;
    'page: for page_list  in tmp {
        for constraint in constraint.as_slice() {
            if constraint.validate(&page_list).is_some() {
                pages.push(page_list);
                continue 'page;
            }
        }

        sum += page_list[page_list.len() / 2]
    }

    sum
}

fn fix(list: &mut [u32], constraint: &[Constraint]) {
    while let Some((x, y)) = constraint.iter().find_map(|c|c.validate(list)) {
        let tmp = list[x];
        list[x] = list[y];
        list[y] = tmp;
    }
}

fn part2(input: (Vec<Constraint>, Vec<Vec<u32>>)) -> u32 {
    let (constraints, broken) = input;
    let mut sum = 0;
    for mut list in broken {
        fix(&mut list, constraints.as_slice());
        sum += list[list.len() / 2];
    }

    sum
}

parsed_day!(
    nom_parsed_bytes(separated_pair(many1(parse_constraints), line_ending, many1(parse_pagelist))),
    |i|part1(i),
    part2
);