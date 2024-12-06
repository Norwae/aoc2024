use std::collections::HashSet;
use crate::*;
use crate::collections::{ArrayBag, CompassDirection, Index2D, IndexMap};
use crate::day::{parse_graphical_input, Day};

#[derive(Debug, Default)]
struct Guard {
    facing: CompassDirection,
    position: Index2D,
}

#[derive(Debug, Default)]
struct Day6 {
    guard: Guard,
    obstacles_per_row: IndexMap<ArrayBag<usize, 32>, 256>,
    obstacles_per_column: IndexMap<ArrayBag<usize, 32>, 256>,
    max_row: usize,
    max_column: usize,
}

impl Day6 {
    fn walk_done(&self) -> bool {
        match self.guard.facing {
            CompassDirection::NORTH => self.guard.position.row == 0,
            CompassDirection::EAST => self.guard.position.column == self.max_column,
            CompassDirection::SOUTH => self.guard.position.row == self.max_row,
            CompassDirection::WEST => self.guard.position.column == 0
        }
    }

    fn do_step(&mut self, turns: usize){
        if turns == 4 { panic!("Spinning in place") }
        let next = self.guard.position + self.guard.facing;
        if let Some(obstacles) = self.obstacles_per_row.get(next.row) {
            let slice = obstacles.as_ref();
            if slice.contains(&next.column) {
                self.guard.facing = self.guard.facing.turn_right();
                self.do_step(turns + 1);
                return;
            }
        }
        self.guard.position = next;
    }

    fn step(&mut self) {
        self.do_step(0)
    }
}

impl Day6 {

}

fn parse(input: &[u8]) -> Result<Day6, !> {
    let mut result = Day6::default();

    parse_graphical_input(input, |byte, position| {
        if byte == b'^' {
            result.guard = Guard { facing: CompassDirection::NORTH, position }
        } else if byte == b'#' {
            result.obstacles_per_row.get_or_insert_default(position.row).insert(position.column);
            result.obstacles_per_column.get_or_insert_default(position.column).insert(position.row);
        }
        result.max_row = result.max_row.max(position.row);
        result.max_column = result.max_column.max(position.column)
    });

    Ok(result)
}


fn part1(mut day: Day6) -> usize {
    let mut visited = HashSet::<Index2D>::new();
    visited.insert(day.guard.position);
    while !day.walk_done() {
        day.step();
        visited.insert(day.guard.position);
    }

    visited.len()
}


parsed_day!(parse, part1);