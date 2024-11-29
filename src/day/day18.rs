use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{line_ending, one_of, space1};
use nom::combinator::map;
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::tuple;
use crate::*;
use crate::day::nom_parsed;
use crate::parse_helpers::parse_signed_nr;

#[derive(Debug, Clone, Eq, PartialEq)]
enum Value {
    RegisterReference(usize),
    Literal(i64),
}
#[derive(Debug, Clone, Eq, PartialEq)]
struct Register(usize);

#[derive(Debug, Clone, Eq, PartialEq)]
enum Command {
    Send(Register),
    Set(Register, Value),
    Add(Register, Value),
    Mul(Register, Value),
    Mod(Register, Value),
    Receive(Register),
    JumpGreaterZero(Value, Value),
}

#[derive(Debug)]
struct VMBaseState {
    registers: Vec<i64>,
    pc: usize,
}

#[derive(Debug)]
enum SendOrReceive<'a> {
    Send(&'a mut i64),
    Receive(&'a mut i64),
}
impl VMBaseState {
    fn resolve(&self, value: &Value) -> i64 {
        match value {
            Value::RegisterReference(rnum) => {
                self.registers[(*rnum) as usize]
            }
            Value::Literal(value) => {
                *value
            }
        }
    }

    fn run(&mut self, program: &[Command], mut send_recv_callback: impl FnMut(SendOrReceive) -> bool) {
        loop {
            let mut next_pc = self.pc + 1;
            match &program[self.pc] {
                Command::Send(Register(off)) => {
                    let target = &mut self.registers[*off];
                    if send_recv_callback(SendOrReceive::Send(target)) {
                        return;
                    }
                }
                Command::Set(Register(offset), v) => self.registers[*offset] = self.resolve(v),
                Command::Add(Register(offset), v) => self.registers[*offset] += self.resolve(v),
                Command::Mul(Register(offset), v) => self.registers[*offset] *= self.resolve(v),
                Command::Mod(Register(offset), v) => self.registers[*offset] %= self.resolve(v),
                Command::Receive(Register(off)) => {
                    let target = &mut self.registers[*off];
                    if send_recv_callback(SendOrReceive::Receive(target)) {
                        return;
                    }
                }
                Command::JumpGreaterZero(reference_value, offset) => {
                    let reference_value = self.resolve(reference_value);
                    let offset = self.resolve(offset);

                    if reference_value > 0 {
                        next_pc = ((self.pc as isize) + (offset as isize)).try_into().expect("Can reconvert to usize")
                    }
                }
            }

            self.pc = next_pc
        }
    }
}

impl Default for VMBaseState {
    fn default() -> Self {
        Self { registers: vec![0; 26], pc: 0 }
    }
}

fn parse_register(input: &str) -> IResult<&str, Register> {
    map(
        one_of("abcdefghijklmnopqrstuvwxy"),
        |r| Register((r as u8 - b'a') as usize),
    )(input)
}

fn parse_value(input: &str) -> IResult<&str, Value> {
    alt((
        map(parse_register, |Register(name)| Value::RegisterReference(name)),
        map(parse_signed_nr::<i64>, |value| Value::Literal(value))
    ))(input)
}

fn parse_binary_command<'a>(mnemonic: &'static str, mut construct: impl FnMut(Value, Value) -> Command) ->
impl FnMut(&'a str) -> IResult<&'a str, Command, nom::error::Error<&'a str>> {
    map(tuple((
        tag(mnemonic),
        space1,
        parse_value,
        space1,
        parse_value
    )), move |(_, _, v1, _, v2)| construct(v1, v2))
}

fn parse_arithmetic_command<'a>(mnemonic: &'static str, mut construct: impl FnMut(Register, Value) -> Command) ->
impl FnMut(&'a str) -> IResult<&'a str, Command, nom::error::Error<&'a str>> {
    map(tuple((
        tag(mnemonic),
        space1,
        parse_register,
        space1,
        parse_value
    )), move |(_, _, reg, _, val)| construct(reg, val))
}

fn parse_unary_command<'a>(mnemonic: &'static str, mut construct: impl FnMut(Register) -> Command) ->
impl FnMut(&'a str) -> IResult<&'a str, Command, nom::error::Error<&'a str>> {
    map(tuple((
        tag(mnemonic),
        space1,
        parse_register
    )), move |(_, _, reg)| construct(reg))
}

fn parse_command(input: &str) -> IResult<&str, Command> {
    alt((
        parse_unary_command("snd", |v| Command::Send(v)),
        parse_arithmetic_command("set", |r, v| Command::Set(r, v)),
        parse_arithmetic_command("add", |r, v| Command::Add(r, v)),
        parse_arithmetic_command("mul", |r, v| Command::Mul(r, v)),
        parse_arithmetic_command("mod", |r, v| Command::Mod(r, v)),
        parse_unary_command("rcv", |v| Command::Receive(v)),
        parse_binary_command("jgz", |r, v| Command::JumpGreaterZero(r, v))
    ))(input)
}

parsed_day!(nom_parsed(separated_list1(line_ending, parse_command)), p1, p2);

fn p1(program: &mut Vec<Command>) -> i64 {
    let mut base = VMBaseState::default();
    let mut recovered = None;
    let mut last_send = 0;
    base.run(program, |sor| {
        match sor {
            SendOrReceive::Send(v) => {
                last_send = *v;
                false
            }
            SendOrReceive::Receive(v) => {
                let value = *v;
                if value != 0 {
                    recovered = Some(last_send);
                    true
                } else {
                    false
                }
            }
        }
    });

    recovered.unwrap()
}


fn perform_duo_vm_step(state: &mut VMBaseState, output_buffer: &mut Vec<i64>, mut input_buffer: &[i64], program: &[Command]) -> usize {
    let mut send_count = 0;
    state.run(program, |sor|{
        match sor {
            SendOrReceive::Send(ptr) => {
                output_buffer.push(*ptr);
                send_count += 1;
                false
            }
            SendOrReceive::Receive(ptr) => {
                if input_buffer.len() == 0 {
                    true
                } else {
                    *ptr = input_buffer[0];
                    input_buffer = &input_buffer[1..];
                    false
                }
            }
        }
    });

    send_count
}

fn p2(program: Vec<Command>) -> usize {
    let mut count = 0;
    let mut state0 = VMBaseState::default();
    let mut state1 = VMBaseState::default();
    let mut buffer_to_vm2 = Vec::new();
    let mut buffer_to_vm1 = Vec::new();

    state1.registers[(b'p' - b'a')  as usize] = 1;

    loop {
        perform_duo_vm_step(&mut state0, &mut buffer_to_vm2, &buffer_to_vm1, &program);
        buffer_to_vm1.clear();
        let fill_for_vm1 = perform_duo_vm_step(&mut state1, &mut buffer_to_vm1, &buffer_to_vm2, &program);
        buffer_to_vm2.clear();
        count += fill_for_vm1;
        if fill_for_vm1 == 0 {
            break
        }
    }

    count
}