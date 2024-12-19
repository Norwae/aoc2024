use crate::collections::{CompassDirection, Index2D};
use crate::day::{nom_parsed_bytes, parse_graphical_input};
use crate::*;
use nom::bytes::complete::take_until;
use nom::{AsBytes, IResult};
use std::rc::Rc;
use fxhash::FxHashSet;

#[derive(Debug, Clone)]
struct Day15 {
    boulders: FxHashSet<Index2D>,
    walls: FxHashSet<Index2D>,
    player: Index2D,
    moves: Rc<Vec<CompassDirection>>,
}


impl Day15 {
    fn expand(&mut self) {
        self.boulders = self.boulders.iter().map(|idx|Index2D{ row: idx.row, column: idx.column * 2}).collect();
        self.walls = self.walls.iter().flat_map(|idx|[Index2D{ row: idx.row, column: idx.column * 2}, Index2D{ row: idx.row, column: idx.column * 2 + 1}]).collect();
        self.player.column *= 2;
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

    fn bump_boulder_east(&mut self, from: Index2D) -> bool {
        let to = from + CompassDirection::EAST;
        let next = to + CompassDirection::EAST;
        debug_assert!(self.boulders.contains(&from) &&
            !self.walls.contains(&to) &&
            !self.boulders.contains(&to));

        if self.walls.contains(&next) {
            false
        } else {
            if !self.boulders.contains(&next) || self.bump_boulder_east(next) {
                self.boulders.remove(&from);
                self.boulders.insert(to);
                true
            } else {
                false
            }
        }
    }
    fn bump_boulder_west(&mut self, from: Index2D) -> bool {
        let to = from + CompassDirection::WEST;
        let next = to + CompassDirection::WEST;
        debug_assert!(self.boulders.contains(&from) && !self.boulders.contains(&to));

        if self.walls.contains(&to) {
            false
        } else {
            if !self.boulders.contains(&next) || self.bump_boulder_west(next) {
                self.boulders.remove(&from);
                self.boulders.insert(to);
                true
            } else {
                false
            }
        }
    }


    fn neighbouring_boulders<'a>(&self, boulder_src_index: Index2D, direction: CompassDirection, target: &'a mut [Index2D;2]) -> &'a [Index2D]  {
        let into_square = boulder_src_index + direction;
        if self.boulders.contains(&into_square) {
            target[0] = into_square;
            return &target[..1];
        }
        let west = boulder_src_index + direction + CompassDirection::WEST;
        let east = boulder_src_index + direction + CompassDirection::EAST;
        if self.boulders.contains(&west) {
            target[0] = west;
            if self.boulders.contains(&east) {
                target[1] = east;
                return &target[..2];
            } else {
                return &target[..1];
            }
        }

        if self.boulders.contains(&east){
            target[0] = east;
            return &target[..1];
        }

        &target[..0]
    }

    fn verify_vertical_move(&self, from: Index2D, direction: CompassDirection) -> bool {
        if self.walls.contains(&(from + direction)) || self.walls.contains(&(from + direction + CompassDirection::EAST)){
            return false;
        }

        let mut neighbours = [Index2D::IMPLAUSIBLE; 2];
        let neighbours = self.neighbouring_boulders(from, direction, &mut neighbours);

        for neighbour in neighbours {
            if !self.verify_vertical_move(*neighbour, direction) {
                return false;
            }
        }
        true
    }

    fn perform_vertical_move(&mut self, from: Index2D, direction: CompassDirection) {
        let mut neighbours = [Index2D::IMPLAUSIBLE; 2];
        let neighbours = self.neighbouring_boulders(from, direction, &mut neighbours);

        for neighbour in neighbours {
            self.perform_vertical_move(*neighbour, direction);
        }

        self.boulders.remove(&from);
        self.boulders.insert(from + direction);
    }

    fn apply_move_2(&mut self, direction: CompassDirection){
        let to = self.player + direction;
        if !self.walls.contains(&to) {
            match direction {
                CompassDirection::NORTH | CompassDirection::SOUTH => {
                    let permit_move = if self.boulders.contains(&to) {
                        if self.verify_vertical_move(to, direction) {
                            self.perform_vertical_move(to, direction);
                            true
                        } else {
                            false
                        }
                    } else {
                        let boulder_src = to + CompassDirection::WEST;
                        if self.boulders.contains(&boulder_src) {
                            if self.verify_vertical_move(boulder_src, direction) {
                                self.perform_vertical_move(boulder_src, direction);
                                true
                            } else {
                                false
                            }
                        } else {
                            true
                        }
                    };

                    if  permit_move {
                      self.player = to;
                    }
                }
                CompassDirection::EAST => {
                    if !self.boulders.contains(&to) || self.bump_boulder_east(to) {
                        self.player = to;
                    }
                }
                CompassDirection::WEST => {
                    let boulder_src = to + CompassDirection::WEST;
                    if !self.boulders.contains(&boulder_src) || self.bump_boulder_west(boulder_src) {
                        self.player = to;
                    }
                }
            }
        }

        debug_assert!(self.map_globally_consistent())
    }

    fn map_globally_consistent(&self) -> bool {
        for boulder in self.boulders.iter() {
            let boulder = *boulder;
            if self.player == boulder {
                return false;
            }
            if self.walls.contains(&boulder) {
                return false;
            }
            if self.boulders.contains(&(boulder + CompassDirection::WEST)) {
                return false;
            }
            if self.boulders.contains(&(boulder + CompassDirection::EAST)) {
                return false;
            }
        }

        true
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


fn parse_maze(input: &[u8]) -> (FxHashSet<Index2D>, FxHashSet<Index2D>, Index2D) {
    let mut boulders = FxHashSet::default();
    let mut walls = FxHashSet::default();
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

fn dbg_print(day: &Day15) {
    let mut buffer = String::new();
    let rows = 0..=day.walls.iter().map(|i| i.row).max().unwrap();
    let cols = 0..=day.walls.iter().map(|i| i.column).max().unwrap();
    for row in rows {
        for column in cols.clone() {
            let idx = Index2D { row, column};
            let char = if day.walls.contains(&idx) {
                '#'
            } else if day.player == idx {
                '@'
            } else if day.boulders.contains(&idx) {
                '['
            } else if day.boulders.contains(&(idx + CompassDirection::WEST)){
                ']'
            } else {
                '.'
            };
            buffer.push(char);
        }
        buffer.push('\n');
    }
    println!("{}", buffer);
}

fn solve2(mut day: Day15) -> usize {
    day.expand();
    for movement in day.moves.clone().iter() {
        day.apply_move_2(*movement);
    }

    day.gps_score()
}

parsed_day!(nom_parsed_bytes(parse), solve1, solve2);
