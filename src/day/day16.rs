use crate::day::nom_parsed;
use crate::*;
use nom::branch::alt;
use nom::bytes::complete::{tag, take};
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::IResult;
use crate::parse_helpers::parse_unsigned_nr;

enum Instruction {
    Rotate(u32),
    SwapByPosition(usize, usize),
    SwapByName(u8, u8),
}

fn parse_rotate(input: &str) -> IResult<&str, Instruction> {
    map(tuple((
        tag("s"),
        parse_unsigned_nr
    )), |(_, n)| Instruction::Rotate(n))(input)
}

fn parse_swap_by_position(input: &str) -> IResult<&str, Instruction> {
    map(tuple((
        tag("x"),
        parse_unsigned_nr,
        tag("/"),
        parse_unsigned_nr
    )), |(_, a, _, b)| Instruction::SwapByPosition(a, b))(input)
}

fn parse_swap_by_name(input: &str) -> IResult<&str, Instruction> {
    map(tuple((
        tag("p"),
        take::<usize, &str, _>(1usize),
        tag("/"),
        take::<usize, &str, _>(1usize)
    )), |(_, a, _, b)| Instruction::SwapByName(a.as_bytes()[0] - b'a', b.as_bytes()[0] - b'a'))(input)
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    alt((parse_rotate, parse_swap_by_name, parse_swap_by_position))(input)
}

struct Program {
    instructions: Vec<Instruction>,
    step: usize,
    run_history: Vec<u64>
}

impl Program {
    const MASKS: [u64; 16] = [
        0xf000000000000000,
        0x0f00000000000000,
        0x00f0000000000000,
        0x000f000000000000,
        0x0000f00000000000,
        0x00000f0000000000,
        0x000000f000000000,
        0x0000000f00000000,
        0x00000000f0000000,
        0x000000000f000000,
        0x0000000000f00000,
        0x00000000000f0000,
        0x000000000000f000,
        0x0000000000000f00,
        0x00000000000000f0,
        0x000000000000000f,
    ];

    fn find_nibble_index(nibble: u8, mut word: u64) -> usize {
        let nib_word = (nibble as u64) << 60;
        let mut idx = 0;

        while (word & Self::MASKS[0]) != nib_word {
            idx += 1;
            word <<= 4;
        }

        idx
    }

    fn swap_nibbles_by_position(nib1: usize, nib2: usize, word: u64) -> u64 {

        let value1 = (word & Self::MASKS[nib1]) << (nib1 * 4);
        let value2 = (word & Self::MASKS[nib2]) << (nib2 * 4);
        let replacement = (value1 >> (nib2 * 4)) | (value2 >> (nib1 * 4));
        let blankedbytes = word & !Self::MASKS[nib1] & !Self::MASKS[nib2];
        replacement | blankedbytes
    }
    fn run(&mut self) -> Option<(usize, usize)> {
        let mut dancers = self.run_history[self.step];
        for i in &self.instructions {
            match i {
                Instruction::Rotate(len) => {
                    dancers = dancers.rotate_right(4 * len)
                }
                Instruction::SwapByPosition(p1, p2) => {
                    dancers = Self::swap_nibbles_by_position(*p1, *p2, dancers)
                }
                Instruction::SwapByName(n1, n2) => {
                    let nibble1 = *n1;
                    let nibble2 = *n2;
                    let p1 = Self::find_nibble_index(nibble1, dancers);
                    let p2 = Self::find_nibble_index(nibble2, dancers);
                    dancers = Self::swap_nibbles_by_position(p1, p2, dancers);
                }
            }
        }

        match self.run_history.iter().position(|prev|*prev == dancers) {
            None => {
                self.run_history.push(dancers);
                self.step += 1;
                None
            }
            Some(v) => {
                Some((v, self.step + 1))
            }
        }
    }

    fn state_at_formatted(&self, index: usize) -> String {
        let state = self.run_history[index];
        Self::format_state(state)
    }

    fn format_state(state: u64) -> String {
        let number_bytes = state.to_be_bytes();
        let mut string_bytes = [b'a'; 16];
        for i in (0..16).step_by(2) {
            string_bytes[i] += (number_bytes[i / 2] & 0xf0) >> 4;
            string_bytes[i+1] += number_bytes[i / 2] & 0x0f;
        }

        String::from_utf8_lossy(&string_bytes).to_string()
    }

    fn last_formatted(&self) -> String {
        self.state_at_formatted(self.step)
    }
}

fn parse_prg(input: &str) -> IResult<&str, Program> {
    map(separated_list1(tag(","), parse_instruction), |instructions|{
        let step = 0;
        let state = 0x0123456789abcdefu64;
        let  run_history = vec![state];
        Program{
            step,
            run_history,
            instructions
        }
    })(input)
}

parsed_day!(nom_parsed(parse_prg), p1, p2);

fn p1(i: &mut Program) -> String {
    i.run();
    i.last_formatted()
}

fn p2(mut i: Program) -> String {
    let mut complete = None;
    while complete.is_none() {
        complete = i.run();
    }
    let (prefix, cycle_length) = complete.unwrap();
    let base = 1000000000;
    let time_spend_in_cyles = base - prefix;
    let cycle_offset = time_spend_in_cyles % cycle_length;

    i.state_at_formatted(prefix + cycle_offset)
}