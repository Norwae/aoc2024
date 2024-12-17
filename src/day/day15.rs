use std::collections::HashSet;
use crate::collections::{CompassDirection, Index2D, Vec2D};
use crate::day::{nom_parsed_bytes, parse_graphical_input, parse_graphical_input_raw, Day};
use crate::*;
use nom::bytes::complete::take_until;
use nom::{AsBytes, IResult};
use std::mem::swap;
use std::rc::Rc;


#[derive(Debug, Clone)]
struct Day15 {
    boulders: HashSet<Index2D>,
    walls: HashSet<Index2D>,
    player: Index2D,
    moves: Rc<Vec<CompassDirection>>,
}


impl Day15 {
    fn expand(&mut self) {
        self.boulders = self.boulders.iter().map(|idx|Index2D{ row: idx.row, column: idx.column * 2}).collect();
        self.walls = self.boulders.iter().flat_map(|idx|[Index2D{ row: idx.row, column: idx.column * 2}, Index2D{ row: idx.row, column: idx.column * 2 + 1}]).collect();
    }

    fn bump_boulder(&mut self, from: Index2D, direction: CompassDirection) -> bool {
        assert!(self.boulders.contains(&from));
        let to = from + direction;
        if !self.walls.contains(&to) {
            if !self.boulders.contains(&to) || self.bump_boulder(to, direction) {
                self.boulders.remove(&from);
                self.boulders.insert(to);
                return true
            }
        }
        false
    }

    fn apply_move_2(&mut self, direction: CompassDirection){
        let to = self.player + direction;
        if !self.walls.contains(&to) {
            let is_free_left = !self.boulders.contains(&to);
            let is_free_right = !self.boulders.contains(&(to + CompassDirection::WEST));

            if is_free_left && is_free_right {
                self.player = to;
            } else if !is_free_left {
            }
        }
    }

    fn gps_score(&self) -> usize {
        self.boulders.iter().fold(0, |acc, idx| acc + 100 * idx.row + idx.column)
    }
    fn apply_move(&mut self, direction: CompassDirection){
        let to = self.player + direction;
        if !self.walls.contains(&to) {
            if !self.boulders.contains(&to) || self.bump_boulder(to, direction) {
                self.player = to;
            }
        }
    }
}


fn parse_maze(input: &[u8]) -> (HashSet<Index2D>, HashSet<Index2D>, Index2D) {
    let mut boulders = HashSet::new();
    let mut walls = HashSet::new();
    let mut player= Index2D::IMPLAUSIBLE;
    parse_graphical_input(input, |byte, idx| {
        match byte {
            b'O' => _ =  boulders.insert(idx),
            b'#' => _ = walls.insert(idx),
            b'@' => {
                player = idx;
            }
            _ => ()
        }
    });

    (boulders, walls, player)
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

fn parse(input: &[u8]) -> IResult<&[u8], Day15> {
    let (rest, maze_without_last_linebreak) = take_until(b"\n\n".as_bytes())(input)?;
    let (boulders, walls, player) = parse_maze(&input[..maze_without_last_linebreak.len() + 1]);
    let moves = Rc::new(parse_moves(&rest[2..]));

    Ok((
        &b"".as_bytes(),
        Day15 {
            boulders, walls, player, moves
        },
    ))
}

fn solve1(day: &mut Day15) -> usize {
    let mut day = day.clone();
    for movement in day.moves.clone().iter() {
        day.apply_move(*movement);
    }

    day.gps_score()
}

fn solve2(mut day: Day15) -> usize {
    day.expand();

    11
}

parsed_day!(nom_parsed_bytes(parse), solve1, solve2);
