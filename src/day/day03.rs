use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::value;
use nom::IResult;
use nom::sequence::{delimited, separated_pair};
use crate::*;
use crate::day::nom_parsed;
use crate::parse_helpers::parse_signed_nr;

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Mul(i32, i32),
    Enable(bool),
}

fn parse_mul(input: &str) -> IResult<&str, Instruction> {
    let (rest, (x, y)) = delimited(
        tag("mul("),
        separated_pair(parse_signed_nr, tag(","), parse_signed_nr),
        tag(")"),
    )(input)?;

    Ok((rest, Instruction::Mul(x, y)))
}

fn parse_toggle(input: &str) -> IResult<&str, Instruction> {
    alt((
        value(Instruction::Enable(true), tag("do()")),
        value(Instruction::Enable(false), tag("don't()"))
    ))(input)
}

fn parse(mut input: &str) -> IResult<&str, Vec<Instruction>> {
    let mut result = Vec::new();
    let mut parse_instruction = alt((parse_mul, parse_toggle));
    while !input.is_empty() {
        if let Ok((rest, inst)) = parse_instruction(input) {
            input = rest;
            result.push(inst)
        } else {
            input = &input[1..]
        }
    }

    Ok((input, result))
}

parsed_day!(nom_parsed(parse), |v| {
    v.into_iter().fold(0, |accu, i| {
        if let Instruction::Mul(x, y) = i {
            accu + *x * *y
        } else {
            accu
        }
    })
}, |v|v.into_iter().fold((0, true), |(accu, active), i|{
    match i {
        Instruction::Mul(x, y) => (accu + if active {x * y} else {0}, active),
        Instruction::Enable(v) => (accu,v),
    }
}).0);