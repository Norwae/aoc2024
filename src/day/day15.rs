use crate::collections::{CompassDirection, Index2D, Vec2D};
use crate::day::{nom_parsed_bytes, parse_graphical_input_raw};
use crate::*;
use std::mem::swap;
use nom::bytes::complete::take_until;
use nom::{AsBytes, IResult};

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum SquareContents {
    Empty,
    MobileBox,
    LockedBox,
    Wall,
}

#[derive(Debug)]
struct Day15 {
    contents: Vec2D<SquareContents>,
    player_start: Index2D,
    moves: Vec<CompassDirection>,
}

fn parse_maze(input: &[u8]) -> (Vec2D<SquareContents>, Index2D) {
    let mut builder = None::<Vec2D<SquareContents>>;
    let mut buffer = Vec::new();
    let mut player_start = Index2D::IMPLAUSIBLE;
    parse_graphical_input_raw(input, |byte, idx| {
        match byte {
            b'.' => buffer.push(SquareContents::Empty),
            b'O' => buffer.push(SquareContents::MobileBox),
            b'#' => buffer.push(SquareContents::Wall),
            b'@' => {
                player_start = idx;
                buffer.push(SquareContents::Empty);
            },
            b'\n' => {
                let mut tmp = Vec::new();
                swap(&mut tmp, &mut buffer);
                if builder.is_none() {
                    builder = Some(tmp.into());
                } else {
                    builder.as_mut().unwrap().extend_from(tmp)
                }
                return true;
            }
            _ => (),
        }

        false
    });

    (builder.unwrap(), player_start)
}

fn parse_moves(input: &[u8]) -> Vec<CompassDirection> {
    let mut buffer = Vec::new();
    for byte in input {
        match byte {
            b'^' => buffer.push(CompassDirection::NORTH),
            b'v' => buffer.push(CompassDirection::SOUTH),
            b'<' => buffer.push(CompassDirection::WEST),
            b'>' => buffer.push(CompassDirection::EAST),
            _ => ()
        }
    }

    buffer
}

fn parse(input: &[u8]) -> IResult<&[u8],Day15> {
    let (rest, maze_without_last_linebreak) = take_until(b"\n\n".as_bytes())(input)?;
    let (contents, player_start) = parse_maze(&input[..maze_without_last_linebreak.len() + 1]);
    let moves = parse_moves(&rest[2..]);

    Ok((&b"".as_bytes(), Day15 { contents, moves, player_start }))
}

parsed_day!(nom_parsed_bytes(parse));
