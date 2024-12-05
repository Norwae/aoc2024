use std::collections::{HashMap, HashSet};
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
use crate::timed::time_span;

struct Constraint {
    left: u8,
    right: u8,
}


struct TokenOrdering {
    lookup_positions: [usize; 100],
}
struct Day5 {
    constraints: Vec<Constraint>,
    token_lists: Vec<Vec<u8>>,
    token_orderings_per_input: Vec<TokenOrdering>,
}

impl Constraint {
    fn validate(&self, pages: &[u8]) -> Option<(usize, usize)> {
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
        lookup_positions[*v as usize] = idx;
    }

    TokenOrdering { lookup_positions }
}

fn part1(input: &mut Day5) -> u32 {
    let Day5 { constraints, token_lists, token_orderings_per_input } = input;
    let mut tmp = Vec::new();
    swap(&mut tmp, token_lists);
    let mut sum = 0;
    'page_list: for page_list in tmp {
        let constraint = build_token_ordering(&page_list, constraints);
        for page_nr in page_list.windows(2) {
            let [l, r] = page_nr else { panic!("windows() is broken") };
            if constraint.lookup_positions[*l as usize] > constraint.lookup_positions[*r as usize] {
                token_lists.push(page_list);
                token_orderings_per_input.push(constraint);
                continue 'page_list;
            }
        }

        sum += page_list[page_list.len() / 2] as u32
    }

    sum
}

fn part2(input: Day5) -> u32 {
    let Day5 { token_lists, token_orderings_per_input, .. } = input;
    let mut sum = 0;
    for (mut list, ordering) in token_lists.into_iter().zip(token_orderings_per_input.into_iter()) {
        list.sort_by(|x, y| ordering.lookup_positions[*x as usize].cmp(&ordering.lookup_positions[*y as usize]));
        sum += list[list.len() / 2] as u32;
    }

    sum
}

parsed_day!(
    nom_parsed_bytes(map(separated_pair(many1(parse_constraints), line_ending, many1(parse_pagelist)), |(constraints,token_lists)|{
        Day5 { constraints, token_lists, token_orderings_per_input: Vec::new()}
    })),
    |i|part1(i),
    part2
);