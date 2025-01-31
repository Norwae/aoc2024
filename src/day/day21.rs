use std::fmt::format;
use clap::Command;
use nom::bytes::complete::take_while1;
use nom::character::complete::one_of;
use nom::Err::Error;
use nom::{error_position, IResult};
use nom::error::ErrorKind;
use crate::*;
use crate::day::day21::CommandPadKey::*;
use crate::day::day21::NumPadKey::*;

trait CommandEncoding<TargetAlphabet> {
    fn commands_to_input(&self, commands: &[TargetAlphabet]) -> Vec<CommandPadKey>;
}
#[derive(Debug, Default, Copy, Clone)]
#[repr(u8)]
enum NumPadKey {
    #[default]
    _A,
    _0,
    _1,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    _9,
}

#[derive(Debug, Default, Copy, Clone)]
enum CommandPadKey {
    #[default]
    A,
    Up,
    Left,
    Down,
    Right,
}

struct CommandPadInput(CommandPadKey, usize);

struct Move {
    reorder_allowed: bool,
    input_1: CommandPadInput,
    input_2: CommandPadInput,
}


const fn zero() -> Move {
    Move { input_1: left(0), input_2: left(0), reorder_allowed: false}
}

const fn uni(input: CommandPadInput) -> Move {
    Move { input_1:  input, input_2: left(0), reorder_allowed: false}
}

const fn strict(input_1: CommandPadInput, input_2: CommandPadInput) -> Move {
    Move { input_1, input_2, reorder_allowed: false }
}

const fn lax(input_1: CommandPadInput, input_2: CommandPadInput) -> Move {
    Move { input_1, input_2, reorder_allowed: true }
}

const fn left(n: usize) -> CommandPadInput {
    CommandPadInput(Left, n)
}

const fn right(n: usize) -> CommandPadInput {
    CommandPadInput(Right, n)
}

const fn up(n: usize) -> CommandPadInput {
    CommandPadInput(Up, n)
}

const fn down(n: usize) -> CommandPadInput {
    CommandPadInput(Down, n)
}

const SHORTEST_PATHS_ON_NUMPAD: [[Move; 11]; 11] = const {
    use CommandPadKey::*;
    let from_A = [
        zero(), // to A
        uni(left(1)), // to 0

        strict(up(1), left(2)), // to 1
        strict(up(1), left(1)), // to 2
        uni(up(1)), // to 3

        strict(up(2), left(2)), // to 4
        strict(up(2), left(1)), // to 5
        uni(up(2)), // to 6

        strict(up(3), left(2)), // to 7
        strict(up(3), left(1)), // to 8
        uni(up(3)), // to 9
    ];
    let from_0 = [
        uni(right(1)), // to A
        zero(), // to 0

        strict(up(1), left(1)), // to 1
        uni(up(1)), // to 2
        lax(up(1), right(1)), // to 3

        strict(up(2), left(1)), // to 4
        uni(up(2)), // to 5
        lax(up(2), right(1)), // to 6

        strict(up(3), left(1)), // to 7
        uni(up(3)), // to 8
        lax(up(3), right(0)), // to 9
    ];
    let from_1 = [
        strict(right(2), down(1)), // to A
        strict(right(1), down(1)), // to 0

        zero(), // to 1
        uni(right(1)), // to 2
        uni(right(2)), // to 3

        uni(up(1)), // to 4
        lax(up(1), right(1)), // to 5
        lax(up(1), right(2)), // to 6

        uni(up(2)), // to 7
        lax(up(2), right(1)), // to 8
        lax(up(2), right(2)), // to 9
    ];
    let from_2 = [
        lax(right(1), down(1)), // to A
        uni(down(1)), // to 0

        uni(left(1)), // to 1
        zero(), // to 2
        uni(right(1)), // to 3

        lax(up(1), left(1)), // to 4
        uni(up(1)), // to 5
        lax(up(1), right(1)), // to 6

        lax(up(2), left(1)), // to 7
        uni(up(2)), // to 8
        lax(up(2), right(1)), // to 9
    ];

    let from_3 = [
        uni(down(1)), // to A
        lax(left(1), down(1)), // to 0

        uni(left(2)), // to 1
        uni(left(1)), // to 2
        zero(), // to 3

        lax(up(1), left(2)), // to 4
        lax(up(1), left(1)), // to 5
        uni(up(1)), // to 6

        lax(up(2), left(2)), // to 7
        lax(up(2), left(1)), // to 8
        uni(up(2)), // to 9
    ];

    let from_4 = [
        strict(right(2), down(2)), // to A
        strict(right(1), down(2)), // to 0

        lax(left(1), down(1)), // to 1
        uni(down(1)), // to 2
        lax(right(1), down(1)), // to 3

        zero(), // to 4
        uni(right(1)), // to 5
        uni(right(2)), // to 6

        uni(up(1)), // to 7
        lax(up(1), right(1)), // to 8
        lax(up(1), right(2)), // to 9
    ];
    let from_5 = [
        strict(right(1), down(2)), // to A
        uni(down(2)), // to 0

        lax(left(1), down(1)), // to 1
        uni(down(1)), // to 2
        lax(right(1), down(1)), // to 3

        uni(left(1)), // to 4
        zero(), // to 5
        uni(right(1)), // to 6

        lax(up(1), left(1)), // to 7
        uni(up(1)), // to 8
        lax(up(1), right(1)), // to 9
    ];

    let from_6 = [
        uni(down(2)), // to A
        lax(left(1), down(2)), // to 0

        lax(left(2), down(1)), // to 1
        lax(down(1), left(1)), // to 2
        uni(down(1)), // to 3

        uni(left(2)), // to 4
        uni(left(1)), // to 5
        zero(), // to 6

        lax(up(1), left(2)), // to 7
        lax(up(1), left(1)), // to 8
        uni(up(1)), // to 9
    ];


    // TODO FROM HERE
    let from_7 = [
        strict(right(2), down(2)), // to A
        strict(right(1), down(2)), // to 0

        lax(left(1), down(1)), // to 1
        uni(down(1)), // to 2
        lax(right(1), down(1)), // to 3

        zero(), // to 4
        uni(right(1)), // to 5
        uni(right(2)), // to 6

        uni(up(1)), // to 7
        lax(up(1), right(1)), // to 8
        lax(up(1), right(2)), // to 9
    ];
    let from_8 = [
        strict(right(1), down(2)), // to A
        uni(down(2)), // to 0

        lax(left(1), down(1)), // to 1
        uni(down(1)), // to 2
        lax(right(1), down(1)), // to 3

        uni(left(1)), // to 4
        zero(), // to 5
        uni(right(1)), // to 6

        lax(up(1), left(1)), // to 7
        uni(up(1)), // to 8
        lax(up(1), right(1)), // to 9
    ];

    let from_9 = [
        uni(down(2)), // to A
        lax(left(1), down(2)), // to 0

        lax(left(2), down(1)), // to 1
        lax(down(1), left(1)), // to 2
        uni(down(1)), // to 3

        uni(left(2)), // to 4
        uni(left(1)), // to 5
        zero(), // to 6

        lax(up(1), left(2)), // to 7
        lax(up(1), left(1)), // to 8
        uni(up(1)), // to 9
    ];

    [from_A, from_0, from_1, from_2, from_3, from_4, from_5, from_6, from_7, from_8, from_9]
};



unimplemented_day!();