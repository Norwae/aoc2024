use std::sync::Arc;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::combinator::map;
use nom::IResult;
use nom::multi::{many1, separated_list1};
use nom::sequence::{separated_pair, terminated};
use crate::*;
use crate::collections::{ArrayBag, IndexMap};
use crate::day::nom_parsed_bytes;
use crate::parse_helpers::parse_unsigned_nr_bytes;
use crate::worker::parallelize;

#[derive(Clone)]
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
    let mut build = IndexMap::<ArrayBag<u8, 32>, 100>::new();
    let mut order = Vec::new();
    for constraint in rules {
        if tokens.contains(&constraint.left) && tokens.contains(&constraint.right) {
            let left = build.get_or_insert_default(constraint.left as usize);
            left.insert(constraint.right);
            build.get_or_insert_default(constraint.right as usize);
        }
    }

    while !build.is_empty() {
        let idx = build.iter().find_map(|(k, v)| {
            if v.is_empty() { Some(k) } else { None }
        }).expect("No circularity allowed");
        build.remove(idx);
        for values in build.values_iter_mut() {
            values.remove(&(idx as u8));
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

fn part1((constraints, token_lists):  (Vec<Constraint>, Vec<Vec<u8>>)) -> String {
    let mut sum_1 = 0i32;
    let mut sum_2 = 0i32;

    let constraints = Arc::new(constraints);

    let tasks = token_lists.into_iter().map(|l|{
        let constraints = constraints.clone();
        move ||{
            let constraint = build_token_ordering(&l, &constraints);
            let mut clone = l.clone();
            clone.sort_by(|l, r| constraint.lookup_positions[*l as usize].cmp(&constraint.lookup_positions[*r as usize]));
            let mid = clone[clone.len() / 2] as i32;
            if clone != l {
                -mid
            } else {
                mid
            }
        }
    });

    for result in parallelize(tasks) {
        if result < 0 {
            sum_2 -= result;
        } else {
            sum_1 += result;
        }
    }

    format!("{sum_1} / {sum_2}")
}

parsed_day!(
    nom_parsed_bytes(separated_pair(many1(parse_constraints), line_ending, many1(parse_pagelist))),
    part1
);