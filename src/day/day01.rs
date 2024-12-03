use std::collections::HashMap;
use std::mem::swap;
use nom::character::complete::{line_ending, space1};
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::{separated_pair};
use crate::*;
use crate::day::{nom_byte_parsed, nom_parsed};
use crate::parse_helpers::{parse_unsigned_nr, parse_unsigned_nr_bytes};

fn parse(mut input: &[u8]) -> IResult<&[u8], (Vec<i32>, Vec<i32>)> {
    let mut switch = false;
    let mut v1 = Vec::new();
    let mut v2 = Vec::new();

    while !input.is_empty() {
        if let Ok((rest, next)) = parse_unsigned_nr_bytes(input) {
            input = rest;
            (if switch { &mut v1 } else { &mut v2 }).push(next);
            switch = !switch;
        } else {
            input = &input[1..]
        }
    }
    Ok((&[], (v1, v2)))
}

parsed_day!(nom_byte_parsed(parse),
    |(a, b)|{
        a.sort();
        b.sort();

        a.iter().zip(b.iter()).map(|(a, b)|(a - b).abs()).sum::<i32>()
    },|(l, r)|{
        let mut counts = HashMap::<i32, i32>::new();
        for r in r {
            *counts.entry(r).or_default() += 1;
        }

        l.into_iter().map(|l| l * counts.get(&l).cloned().unwrap_or_default()).sum::<i32>()
    }
);