use std::collections::HashMap;
use nom::character::complete::{line_ending, space1};
use nom::multi::separated_list1;
use nom::sequence::{separated_pair};
use crate::*;
use crate::day::nom_parsed;
use crate::parse_helpers::parse_unsigned_nr;

parsed_day!(nom_parsed(
    separated_list1(line_ending,
        separated_pair(parse_unsigned_nr::<i32>, space1, parse_unsigned_nr::<i32>)
    )),
    |pairs|{
        let (mut a, mut b): (Vec<i32>, Vec<i32>) = pairs.iter().cloned().unzip();
        a.sort();
        b.sort();

        a.iter().zip(b.iter()).map(|(a, b)|(a - b).abs()).sum::<i32>()
    },|pairs|{
        let mut counts = HashMap::<i32, i32>::new();
        for (_, r) in &pairs {
            *counts.entry(*r).or_default() += 1;
        }

        pairs.into_iter().map(|(l,_)| l * counts.get(&l).cloned().unwrap_or_default()).sum::<i32>()
    }
);