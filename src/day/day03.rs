use nom::bytes::complete::tag;
use nom::{AsBytes, IResult};
use nom::sequence::{delimited, separated_pair};
use crate::*;
use crate::day::nom_parsed;
use crate::parse_helpers::{parse_signed_nr, parse_unsigned_nr, parse_unsigned_nr_bytes};

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Mul(i32, i32),
    Enable(bool),
}

fn parse_mul(input: &[u8]) -> IResult<&[u8], Instruction> {
    let (rest, (x, y)) = delimited(
        tag(b"mul("),
        separated_pair(parse_unsigned_nr_bytes, tag(b","), parse_unsigned_nr_bytes),
        tag(b")"),
    )(input)?;

    Ok((rest, Instruction::Mul(x, y)))
}


fn parse(input: &str) -> IResult<&str, Vec<Instruction>> {
    let mut result = Vec::new();
    let mut bytes = input.as_bytes();
    while !bytes.is_empty() {
        if bytes.starts_with(b"do()") {
            bytes = &bytes[4..];
            result.push(Instruction::Enable(true))
        } else if bytes.starts_with(b"don't()") {
            bytes = &bytes[7..];
            result.push(Instruction::Enable(false))
        } else if let Ok((rest, inst)) = parse_mul(bytes) {
            bytes = rest;
            result.push(inst)
        } else {
            bytes = &bytes[1..]
        }
    }

    Ok(("", result))
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