use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{map, value};
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::tuple;
use crate::*;
use crate::day::nom_parsed_bytes;
use crate::parse_helpers::parse_unsigned_nr_bytes;

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
enum Operation {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

#[derive(Debug)]
struct VM {
    register_a: i64,
    register_b: i64,
    register_c: i64,
    instruction_pointer: usize,
    program: Vec<Operation>,
    output: String
}

impl VM {
    fn step(&mut self) {
        let instruction = self.program[self.instruction_pointer];
        let argument = self.program[self.instruction_pointer + 1] as u8;

        match instruction {
            Operation::Adv => {
                self.register_a = self.register_a / (1 << self.lookup_combo_operand(argument));
            }
            Operation::Bxl => {
                self.register_b = self.register_b ^ argument as i64;
            }
            Operation::Bst => {
                self.register_b = self.lookup_combo_operand(argument) & 0x07;
            }
            Operation::Jnz => {
                if self.register_a != 0 {
                    self.instruction_pointer = argument as usize;
                    return;
                }
            }
            Operation::Bxc => {
                self.register_b ^= self.register_c;
            }
            Operation::Out => {
                if !self.output.is_empty() {
                    self.output.push(',');
                }
                self.output.push_str(&format!("{}", self.lookup_combo_operand(argument) & 0x07));
            }
            Operation::Bdv => {
                self.register_b = self.register_a / (1 << self.lookup_combo_operand(argument));
            }
            Operation::Cdv => {
                self.register_c = self.register_a / (1 << self.lookup_combo_operand(argument));
            }
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
            _ => panic!("invalid value")
        }
    }
}

fn parse_instruction(input: &[u8]) -> IResult<&[u8], Operation> {
    alt((
        value(Operation::Adv, tag("0")),
        value(Operation::Bxl, tag("1")),
        value(Operation::Bst, tag("2")),
        value(Operation::Jnz, tag("3")),
        value(Operation::Bxc, tag("4")),
        value(Operation::Out, tag("5")),
        value(Operation::Bdv, tag("6")),
        value(Operation::Cdv, tag("7")),
    ))(input)
}
fn parse(input: &[u8]) -> IResult<&[u8], VM> {
    map(tuple((
        tag("Register A: "),
        parse_unsigned_nr_bytes,
        tag("\nRegister B: "),
        parse_unsigned_nr_bytes,
        tag("\nRegister C: "),
        parse_unsigned_nr_bytes,
        tag("\n\nProgram: "),
        separated_list1(tag(","), parse_instruction)
    )), |(_, register_a, _, register_b, _, register_c, _, program)| VM { register_c, register_b, register_a, program, instruction_pointer: 0, output: String::new()})(input)
}

fn solve1(mut day: VM) -> String {
    while day.instruction_pointer < day.program.len() {
        day.step();
    }

    day.output
}

parsed_day!(nom_parsed_bytes(parse), solve1);