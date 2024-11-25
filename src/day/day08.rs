use std::collections::HashMap;
use std::fmt::format;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, digit1, line_ending, space1};
use nom::combinator::{map, map_res, opt, value};
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::{terminated, tuple};
use crate::*;
use crate::day::nom_parsed;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum ArithOp {
    Inc, Dec
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum ComparisonOp {
    Less,
    LessEqual,
    Equal,
    GreaterEqual,
    Greater,
    NotEqual
}

#[derive(Debug, Clone)]
struct Instruction<'a> {
    target_register: &'a str,
    arithmetic: ArithOp,
    arithmetic_const: i64,
    operand_register: &'a str,
    operand_operation: ComparisonOp,
    operand_const: i64
}

impl <'a> Instruction<'a> {
    fn apply<'b>(&self, data: &'b mut HashMap<&'a str, i64>) -> i64 {
        let comparison_value = data.get(self.operand_register).cloned().unwrap_or_default();
        let result = match self.operand_operation {
            ComparisonOp::Less =>  comparison_value < self.operand_const,
            ComparisonOp::LessEqual =>  comparison_value <= self.operand_const,
            ComparisonOp::Equal => comparison_value == self.operand_const,
            ComparisonOp::GreaterEqual => comparison_value >= self.operand_const,
            ComparisonOp::Greater => comparison_value > self.operand_const,
            ComparisonOp::NotEqual =>  comparison_value != self.operand_const
        };

        let mut target_value = data.get(self.target_register).cloned().unwrap_or_default();
        if result {
            match self.arithmetic {
                ArithOp::Inc => target_value += self.arithmetic_const,
                ArithOp::Dec => target_value -= self.arithmetic_const,
            }

            data.insert(self.target_register, target_value);
        }
        target_value
    }
}

fn parse_i64(input: &str) -> IResult<&str, i64> {
    map_res(tuple((
        opt(tag("-")),
        digit1
    )), |(neg, digits)| {
        let neg: Option<&str> = neg;
        digits.parse::<i64>().map(|v|{
            if neg.is_some() {
                -v
            } else {
                v
            }
        })
    })(input)
}

fn parse(i: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list1(line_ending, parse_instruction)(i)
}

fn parse_instruction(i: &str) -> IResult<&str, Instruction> {
    map(tuple((
        terminated(alpha1, space1),
        terminated(parse_arith_op, space1),
        terminated(parse_i64, space1),
        tag("if "),
        terminated(alpha1, space1),
        terminated(parse_comparison_op, space1),
        parse_i64
    )), |(target_register, arithmetic, arithmetic_const, _, operand_register, operand_operation, operand_const) |{
        Instruction { target_register, arithmetic, arithmetic_const, operand_register, operand_operation, operand_const }
    })(i)
}
 fn parse_arith_op(i: &str) -> IResult<&str, ArithOp> {
    alt((
        value(ArithOp::Inc, tag("inc")),
        value(ArithOp::Dec, tag("dec"))
    ))(i)
}

fn parse_comparison_op(i: &str) -> IResult<&str, ComparisonOp> {
    alt((
        value(ComparisonOp::Equal, tag("==")),
        value(ComparisonOp::LessEqual, tag("<=")),
        value(ComparisonOp::GreaterEqual, tag(">=")),
        value(ComparisonOp::NotEqual, tag("!=")),
        value(ComparisonOp::Less, tag("<")),
        value(ComparisonOp::Greater, tag(">")),
    ))(i)
}

fn part1(input: &mut Vec<Instruction>) -> String {
    let mut registers = HashMap::new();
    let mut global_max = 0;
    for i in input {
        let new_value = i.apply(&mut registers);
        global_max = global_max.max(new_value)
    }

    let final_max = *registers.values().max().unwrap();
    format!("final: {final_max}, global: {global_max}")
}

parsed_day!(nom_parsed(parse), part1, |_|"<see before>");