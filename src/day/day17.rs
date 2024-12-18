use crate::day::nom_parsed_bytes;
use crate::parse_helpers::parse_unsigned_nr_bytes;
use crate::*;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{map, value};
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::IResult;
use std::rc::Rc;

#[derive(Debug, Clone)]
struct VM {
    register_a: i64,
    register_b: i64,
    register_c: i64,
    instruction_pointer: usize,
    program: Rc<Vec<u8>>,
    output: Vec<u8>,
}

impl VM {


    fn step(&mut self) {
        let instruction = self.program[self.instruction_pointer];
        let argument = self.program[self.instruction_pointer + 1] as u8;

        match instruction {
            0 => {
                self.register_a = self.register_a / (1 << self.lookup_combo_operand(argument));
            }
            1 => {
                self.register_b = self.register_b ^ argument as i64;
            }
            2 => {
                self.register_b = self.lookup_combo_operand(argument) & 0x07;
            }
            3 => {
                if self.register_a != 0 {
                    self.instruction_pointer = argument as usize;
                    return;
                }
            }
            4 => {
                self.register_b ^= self.register_c;
            }
            5 => {
                self.output
                    .push((self.lookup_combo_operand(argument) & 0x07) as u8);
            }
            6 => {
                self.register_b = self.register_a / (1 << self.lookup_combo_operand(argument));
            }
            7 => {
                self.register_c = self.register_a / (1 << self.lookup_combo_operand(argument));
            }
            _ => unreachable!()
        }
        self.instruction_pointer += 2;
    }

    fn lookup_combo_operand(&self, value: u8) -> i64 {
        match value {
            0..=3 => value as i64,
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            7 => panic!("reserved value"),
            _ => panic!("invalid value"),
        }
    }
}

fn parse_instruction(input: &[u8]) -> IResult<&[u8], u8> {
    alt((
        value(0, tag("0")),
        value(1, tag("1")),
        value(2, tag("2")),
        value(3, tag("3")),
        value(4, tag("4")),
        value(5, tag("5")),
        value(6, tag("6")),
        value(7, tag("7")),
    ))(input)
}
fn parse(input: &[u8]) -> IResult<&[u8], VM> {
    map(
        tuple((
            tag("Register A: "),
            parse_unsigned_nr_bytes,
            tag("\nRegister B: "),
            parse_unsigned_nr_bytes,
            tag("\nRegister C: "),
            parse_unsigned_nr_bytes,
            tag("\n\nProgram: "),
            separated_list1(tag(","), parse_instruction),
        )),
        |(_, register_a, _, register_b, _, register_c, _, program)| VM {
            register_c,
            register_b,
            register_a,
            program: Rc::new(program),
            instruction_pointer: 0,
            output: Vec::new(),
        },
    )(input)
}

fn solve(day: VM) -> String {
    let program_length = day.program.len();
    let mut day_part_1 = day.clone();
    while day_part_1.instruction_pointer < day_part_1.program.len() {
        day_part_1.step();
    }

    let output_part_1 = day_part_1.output;

    let mut i = 0usize;
    let mut match_length = 1;
    loop {
        let mut day = day.clone();
        day.register_a = i as i64;

        while day.instruction_pointer < day.program.len() {
            day.step();
        }
        let output = day.output.as_slice();
        let program_suffix = &day.program[program_length - match_length..];
        if output == program_suffix {
            if match_length ==  day.program.len() {
                break;
            }
            match_length += 1;
            i <<= 3;
        } else {
            i += 1;
        }

    }

    format!("Part 1: {output_part_1:?}, Part 2: {i}")
}

parsed_day!(nom_parsed_bytes(parse), solve);
