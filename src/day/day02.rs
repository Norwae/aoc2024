use nom::character::complete::{digit1, line_ending, space1};
use nom::combinator::map_opt;
use nom::IResult;
use nom::multi::separated_list1;
use crate::*;
use crate::day::nom_parsed;

parsed_day!(nom_parsed(parse), part1, part2);

fn part2_line(line: Vec<u32>) -> u32{
    for i in 0..line.len() {
        for j in 0..line.len() {
            if i != j && line[i] % line[j] == 0 {
                return line[i] / line[j]
            }
        }
    }
    unreachable!()
}

fn part2(input: Vec<Vec<u32>>) -> u32 {
    input.into_iter().map(part2_line).sum()
}

fn part1(input: &mut Vec<Vec<u32>>) -> u32 {
    input.into_iter().map(|line|{
        let min = *line.into_iter().min().unwrap();
        let max = *line.into_iter().max().unwrap();
        max - min
    }).sum()
}

fn parse(input: &str) -> IResult<&str, Vec<Vec<u32>>> {
    separated_list1(
        line_ending,
        separated_list1(
            space1,
            map_opt(digit1, |str: &str|str.parse::<u32>().ok())
        )
    )(input)
}