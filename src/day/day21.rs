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
    _9
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


struct Coder {
    numpad_from_to_table: [[Vec<CommandPadKey>; 11]; 11],
    cmdpad_from_to_table: [[Vec<CommandPadKey>; 5]; 5],
}
impl Default for Coder {
    fn default() -> Coder {
        let numpad_from_to_table = [
            [vec![], vec![Left], vec![ Up, Left, Left], vec![Up, Left], vec![Up], vec![Up, Up, Left, Left], vec![Up, Up, Left], vec![Up, Up], vec![Up, Up, Up, Left, Left], vec![Up, Up, Up, Left], vec![Up, Up, Up]],
            [vec![Right], vec![], vec![Up, Left], vec![Up], vec![Up, Right], vec![Up, Up, Left], vec![Up, Up], vec![Up, Up, Right], vec![Up, Up, Up, Left], vec![Up, Up, Up], vec![Up, Up, Up, Right]],


            [vec![Right, Right, Down], vec![Right, Down], vec![], vec![Right], vec![Right, Right], vec![Up], vec![Up, Right], vec![Up, Right, Right], vec![Up, Up], vec![Up, Up, Right], vec![Up, Up, Right, Right]],
            [vec![Right, Down], vec![Down], vec![Left], vec![], vec![Right], vec![Up, Left], vec![Up], vec![Up, Right], vec![Up, Up, Left], vec![Up, Up], vec![Up, Up, Right]],
            [vec![Down], vec![Down, Left], vec![Left, Left], vec![Left], vec![], vec![Up, Left, Left], vec![Up, Left], vec![Up], vec![Up, Up, Left, Left], vec![Up, Up, Left], vec![Up, Up]],

            [vec![Right, Right, Down, Down], vec![Right, Down, Down], vec![Down], vec![Right, Down], vec![Right, Right, Down], vec![], vec![Right], vec![Right, Right], vec![Up], vec![Up, Right], vec![Up, Right, Right]],
            [vec![Right, Down, Down], vec![Down, Down], vec![Down, Left], vec![Down], vec![Right, Down], vec![Left], vec![], vec![Right], vec![Up, Left], vec![Up], vec![Up, Right]],
            [vec![Down, Down], vec![Down, Down, Left], vec![Down, Left, Left], vec![Down, Left], vec![Down], vec![Left, Left], vec![Left], vec![], vec![Up, Left, Left], vec![Up, Left], vec![Up]],

            [vec![Right, Right, Down, Down, Down],  vec![Right, Down, Down, Down], vec![Down, Down], vec![Right, Down, Down],vec![Right, Right, Down, Down],vec![Down], vec![Right, Down],vec![Right, Right, Down], vec![], vec![Right],vec![Right, Right]],
            [vec![Right, Down, Down, Down],vec![Down, Down, Down], vec![Down, Down, Left], vec![Down, Down],vec![Right, Down, Down],vec![Down, Left], vec![Down],vec![Right, Down], vec![Left], vec![],vec![Right]],
            [vec![Down, Down, Down],vec![Down, Down, Down, Left], vec![Down, Down, Left, Left], vec![Down, Down, Left],vec![Down, Down],vec![Down, Left, Left], vec![Down, Left],vec![Down], vec![Left, Left], vec![Left],vec![]],
        ];
        let cmdpad_from_to_table = [
            [vec![], vec![Left], vec![Down, Left, Left], vec![Down, Left], vec![Down]],
            [vec![Right], vec![],vec![Down, Left], vec![Down], vec![Down, Right]],
            [vec![Right, Right, Up], vec![Right, Up], vec![], vec![Right], vec![Right, Right]],
            [vec![Right, Up], vec![Up], vec![Left], vec![], vec![Right]],
            [vec![Up], vec![Left, Up], vec![Left, Left], vec![Left], vec![]]
        ];

        Coder { numpad_from_to_table, cmdpad_from_to_table }
    }
}

impl CommandEncoding<NumPadKey> for Coder {
    fn commands_to_input(&self, commands: &[NumPadKey]) -> Vec<CommandPadKey> {
        let mut position = _A;
        let mut result = Vec::new();
        for command in commands {
            result.extend_from_slice(&self.numpad_from_to_table[position as usize][*command as usize]);
            result.push(A);
            position = *command
        }

        result
    }
}

impl CommandEncoding<CommandPadKey> for Coder {
    fn commands_to_input(&self, commands: &[CommandPadKey]) -> Vec<CommandPadKey> {
        let mut position = A;
        let mut result = Vec::new();
        for command in commands {
            result.extend_from_slice(&self.cmdpad_from_to_table[position as usize][*command as usize]);
            result.push(A);
            position = *command
        }

        result
    }
}

fn parse(mut input: &[u8]) -> IResult<&[u8], (usize, Vec<NumPadKey>)> {
    let mut accu = 0;
    let mut commands = Vec::new();
    while !input.is_empty() {
        let (cmd, v) = match input[0] {
            b'A' => {
                commands.push(_A);
                return Ok((&input[1..], (accu, commands)));
            }
            b'1'=> (_1, 1),
            b'2'=> (_2, 2),
            b'3'=> (_3, 3),
            b'4'=> (_4, 4),
            b'5'=> (_5, 5),
            b'6'=> (_6, 6),
            b'7'=> (_7, 7),
            b'8'=> (_8, 8),
            b'9'=> (_9, 9),
            b'0'=> (_0, 0),
            _ => {
                input = &input[1..];
                continue;
            }
        };
        commands.push(cmd);
        accu = 10 * accu + v;
        input = &input[1..];
    }

    Err(Error(error_position!(input, ErrorKind::Eof)))
}

#[derive(Default)]
struct StreamState {
    coder: Coder,
    accu_1: usize
}

fn process_next_stream_element(state: &mut StreamState, next: (usize, Vec<NumPadKey>)) {
    let (n, commands) = next;
    let coder = &state.coder;
    let codon =  coder.commands_to_input(coder.commands_to_input(coder.commands_to_input(&commands).as_slice()).as_slice());

    state.accu_1 += dbg!(dbg!(n) * dbg!(codon.len()));
}

streaming_day!(parse, process_next_stream_element, |s|s.accu_1);