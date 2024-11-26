use nom::bytes::complete::tag;
use nom::character::complete::{digit1, line_ending};
use nom::combinator::map;
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::{preceded, tuple};
use crate::*;
use crate::day::nom_parsed;
use crate::parse_helpers::parse_unsigned_nr;

fn parse_line(input: &str) -> IResult<&str, Vec<usize>> {
    preceded(
        tuple((digit1, tag(" <-> "))),
        separated_list1(tag(", "), parse_unsigned_nr::<usize>),
    )(input)
}

parsed_day!(nom_parsed(map(
    separated_list1(line_ending, parse_line),
    |input| {
        let length = input.len();
        Day12{input, coverage: vec![false; length]}
    })),
    part1,
    part2);

struct Day12 {
    input: Vec<Vec<usize>>,
    coverage: Vec<bool>,
}

fn part2(day: Day12) -> usize {
    let Day12 { mut coverage, input } = day;
    let mut count = 1;
    while let Some((uncovered,_)) = coverage.iter().enumerate().find(|(_, it)|!**it)  {
        count += 1;
        let mut queue = vec![uncovered];
        while let Some(index) = queue.pop() {
            if !coverage[index] {
                coverage[index] = true;
                queue.extend_from_slice(&input[index]);
            }
        }
    }

    count
}
fn part1(day: &mut Day12) -> usize{
    let Day12 { input, coverage} = day;
    let mut count = 0;
    let mut queue = vec![0];
    while let Some(index) = queue.pop() {
        if !coverage[index] {
            coverage[index] = true;
            count += 1;
            queue.extend_from_slice(&input[index]);
        }
    }

    count
}