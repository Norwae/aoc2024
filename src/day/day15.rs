use crate::collections::{CompassDirection, Index2D, Vec2D};
use crate::day::{nom_parsed_bytes, parse_graphical_input_raw};
use crate::*;
use nom::bytes::complete::take_until;
use nom::{AsBytes, IResult};
use std::mem::swap;
use std::rc::Rc;

trait SquareContents {
    fn is_box(self) -> bool;
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum SquareContentsPart1 {
    Empty,
    MobileBox,
    Wall,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum SquareContentsPart2 {
    Empty,
    BoxLeft,
    BoxRight,
    Wall,
}

impl SquareContents for SquareContentsPart1 {
    fn is_box(self) -> bool {
        self == SquareContentsPart1::MobileBox
    }
}

impl SquareContents for SquareContentsPart2 {
    fn is_box(self) -> bool {
        self == SquareContentsPart2::BoxLeft
    }
}

#[derive(Debug, Clone)]
struct Day15<T: SquareContents> {
    contents: Vec2D<T>,
    player: Index2D,
    moves: Rc<Vec<CompassDirection>>,
}

impl<T: SquareContents + Copy> Day15<T> {
    fn gps_score(&self) -> usize {
        let mut gps = 0;

        for idx in self.contents.indices() {
            if self.contents[idx].is_box() {
                gps += idx.row * 100;
                gps += idx.column;
            }
        }
        gps
    }
}

impl Day15<SquareContentsPart1> {
    fn into_part_2(self) -> Day15<SquareContentsPart2> {
        let new_width = self.contents.row_length() * 2;
        let mut tmp = Vec::with_capacity(new_width * new_width);

        for idx in self.contents.indices() {
            match self.contents[idx] {
                SquareContentsPart1::Empty => {
                    tmp.push(SquareContentsPart2::Empty);
                    tmp.push(SquareContentsPart2::Empty);
                }
                SquareContentsPart1::MobileBox => {
                    tmp.push(SquareContentsPart2::BoxLeft);
                    tmp.push(SquareContentsPart2::BoxRight);
                }
                SquareContentsPart1::Wall => {
                    tmp.push(SquareContentsPart2::Wall);
                    tmp.push(SquareContentsPart2::Wall);
                }
            }
        }

        Day15 {
            contents: Vec2D::new_from_flat(tmp, new_width),
            player: self.player,
            moves: self.moves,
        }
    }
    fn shift_box(&mut self, from: Index2D, direction: CompassDirection) -> bool {
        // Calculate the target position of the box
        let to = from + direction;

        // Check if the movement collides with a wall or locked box
        match self.contents[to] {
            SquareContentsPart1::Wall => false, // Deny the move
            SquareContentsPart1::Empty => {
                // Move the box into the empty space
                self.contents[to] = SquareContentsPart1::MobileBox;
                self.contents[from] = SquareContentsPart1::Empty;
                true // State changed
            }
            SquareContentsPart1::MobileBox => {
                // Attempt to shift the next box
                if self.shift_box(to, direction) {
                    self.contents[to] = SquareContentsPart1::MobileBox;
                    self.contents[from] = SquareContentsPart1::Empty;

                    true // State changed
                } else {
                    false // Unable to shift the next box
                }
            }
        }
    }

    fn apply_move(&mut self, direction: CompassDirection) {
        let into_position = self.player + direction;
        match self.contents[into_position] {
            SquareContentsPart1::Empty => self.player = into_position,
            SquareContentsPart1::MobileBox => {
                if self.shift_box(into_position, direction) {
                    self.player = into_position;
                }
            }
            SquareContentsPart1::Wall => (),
        }
    }
}

impl Day15<SquareContentsPart2> {

}

fn parse_maze(input: &[u8]) -> (Vec2D<SquareContentsPart1>, Index2D) {
    let mut builder = None::<Vec2D<SquareContentsPart1>>;
    let mut buffer = Vec::new();
    let mut player_start = Index2D::IMPLAUSIBLE;
    parse_graphical_input_raw(input, |byte, idx| {
        match byte {
            b'.' => buffer.push(SquareContentsPart1::Empty),
            b'O' => buffer.push(SquareContentsPart1::MobileBox),
            b'#' => buffer.push(SquareContentsPart1::Wall),
            b'@' => {
                player_start = idx;
                buffer.push(SquareContentsPart1::Empty);
            }
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
            _ => (),
        }
    }

    buffer
}

fn parse(input: &[u8]) -> IResult<&[u8], Day15<SquareContentsPart1>> {
    let (rest, maze_without_last_linebreak) = take_until(b"\n\n".as_bytes())(input)?;
    let (contents, player_start) = parse_maze(&input[..maze_without_last_linebreak.len() + 1]);
    let moves = parse_moves(&rest[2..]);

    Ok((
        &b"".as_bytes(),
        Day15 {
            contents,
            moves: Rc::new(moves),
            player: player_start,
        },
    ))
}

fn solve1(day: &mut Day15<SquareContentsPart1>) -> usize {
    let mut day = day.clone();
    for movement in day.moves.clone().iter() {
        day.apply_move(*movement);
    }

    day.gps_score()
}

fn solve2(day: Day15<SquareContentsPart1>) -> usize {
    let mut day = day.into_part_2();
    10
}
parsed_day!(nom_parsed_bytes(parse), solve1, solve2);
