use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::combinator::map;
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use crate::*;
use crate::day::nom_parsed_bytes;
use crate::parse_helpers::parse_unsigned_nr_bytes;

#[derive(Debug)]
struct Problem {
    target_value: u64,
    operands: Vec<u64>,
}

fn concat(l: u64, r: u64) -> u64 {
    let shifting = r.ilog10() + 1;
    let shifted = l * 10_u64.pow(shifting);

    shifted + r
}

fn can_solve(target_value: u64, accu: u64, rest: &[u64], allow_concat: bool) -> bool {
    if rest.is_empty() {
        accu == target_value
    } else {
        let next = rest[0];
        if accu + next > target_value {
            false
        } else {
            can_solve(target_value, accu * next, &rest[1..], allow_concat) ||
                can_solve(target_value, accu + next, &rest[1..], allow_concat) ||
                (allow_concat && can_solve(target_value, concat(accu, next), &rest[1..], true))
        }

    }
}


fn parse(input: &[u8]) -> IResult<&[u8], Vec<Problem>> {
    let parse_problem = map(
        separated_pair(parse_unsigned_nr_bytes, tag(b": "), separated_list1(tag(b" "), parse_unsigned_nr_bytes)),
        |(target_value, operands)| Problem { target_value, operands },
    );
    separated_list1(line_ending, parse_problem)(input)
}

fn solve(input: Vec<Problem>) -> String {
    let mut sum_1 = 0;
    let mut sum_2 = 0;

    for problem in input {
        if can_solve(problem.target_value, problem.operands[0], &problem.operands[1..], false) {
            sum_1 += problem.target_value;
            sum_2 += problem.target_value;
        } else if can_solve(problem.target_value, problem.operands[0], &problem.operands[1..], true) {
            sum_2 += problem.target_value;
        }
    }

    format!("{} - {}", sum_1, sum_2)
}

parsed_day!(nom_parsed_bytes(parse), solve);