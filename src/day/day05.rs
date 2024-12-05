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
    left: u8,
    right: u8,
}


struct TokenOrdering {
    lookup_positions: [usize; 100],
}

fn parse_constraints(input: &[u8]) -> IResult<&[u8], Constraint> {
    map(terminated(separated_pair(parse_unsigned_nr_bytes, tag("|"), parse_unsigned_nr_bytes), line_ending), |(left, right)| Constraint { left, right })(input)
}

fn parse_pagelist(input: &[u8]) -> IResult<&[u8], Vec<u8>> {
    terminated(separated_list1(tag(","), parse_unsigned_nr_bytes), line_ending)(input)
}

fn build_token_ordering(tokens: &[u8], rules: &[Constraint]) -> TokenOrdering {
    let mut build = [const { None::<Vec<u8>> }; 100];
    let mut order = Vec::new();
    for constraint in rules {
        if tokens.contains(&constraint.left) && tokens.contains(&constraint.right) {
            let left = &mut build[constraint.left as usize];
            let left = left.get_or_insert_default();
            left.push(constraint.right);
            let right = &mut build[constraint.right as usize];
            right.get_or_insert_default();
        }
    }

    while !build.iter().all(|it| it.is_none()) {
        let idx = build.iter().enumerate().find_map(|(k, v)| {
            if let Some(v) = v {
                if v.is_empty() {
                    return Some(k);
                }
            }
            None
        }).expect("No circularity allowed");
        build[idx] = None;
        for values in build.iter_mut() {
            if let Some(list) = values {
                if let Some(idx) = list.iter().position(|v| *v as usize == idx) {
                    list.remove(idx);
                }
            }
        }
        order.push(idx);
    }
    order.reverse();
    let mut lookup_positions = [usize::MAX; 100];
    for (idx, v) in order.iter().enumerate() {
        lookup_positions[*v] = idx;
    }

    TokenOrdering { lookup_positions }
}

fn part1((constraints, token_lists): &(Vec<Constraint>, Vec<Vec<u8>>)) -> String {
    let mut sum_1 = 0u32;
    let mut sum_2 = 0u32;
    for page_list in token_lists {
        let constraint = build_token_ordering(&page_list, constraints);
        let mut clone = page_list.clone();
        clone.sort_by(|l, r| constraint.lookup_positions[*l as usize].cmp(&constraint.lookup_positions[*r as usize]));
        let mid = clone[clone.len() / 2];
        let target = if &clone == page_list { &mut sum_1 } else { &mut sum_2 };
        *target += mid as u32;
    }

    format!("Part 1: {sum_1}, Part 2: {sum_2}")
}

parsed_day!(
    nom_parsed_bytes(separated_pair(many1(parse_constraints), line_ending, many1(parse_pagelist))),
    |i|part1(i),
    |_|"<see before>"
);