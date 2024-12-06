use std::collections::{HashMap, HashSet};
use std::mem::swap;
use crate::*;
use crate::collections::{ArrayBag, CompassDirection, Index2D, IndexMap};
use crate::collections::CompassDirection::WEST;
use crate::day::parse_graphical_input;

#[derive(Debug, Default, Clone)]
struct Guard {
    facing: CompassDirection,
    position: Index2D,
}

impl Guard {
    fn step(&mut self) {
        self.position += self.facing;
    }

    fn turn(&mut self) {
        self.facing = self.facing.turn_right();
    }
}

#[derive(Debug, Default, Clone)]
struct Day6 {
    initial_guard: Guard,
    obstacles_per_row: IndexMap<ArrayBag<usize, 32>, 256>,
    obstacles_per_column: IndexMap<ArrayBag<usize, 32>, 256>,
    visited: HashMap<Index2D, ArrayBag<CompassDirection, 4>>,
    max_row: usize,
    max_column: usize,
}

fn parse(input: &[u8]) -> Result<Day6, !> {
    let mut result = Day6::default();

    parse_graphical_input(input, |byte, position| {
        if byte == b'^' {
            result.initial_guard = Guard { facing: CompassDirection::NORTH, position };
            result.visited.entry(position).or_default().insert(CompassDirection::NORTH);
        } else if byte == b'#' {
            result.obstacles_per_row.get_or_insert_default(position.row).insert(position.column);
            result.obstacles_per_column.get_or_insert_default(position.column).insert(position.row);
        }
        result.max_row = result.max_row.max(position.row);
        result.max_column = result.max_column.max(position.column)
    });

    Ok(result)
}

impl Day6 {
    fn step(&mut self, guard: &mut Guard) -> bool {
        guard.step();
        self.visited.entry(guard.position).or_default().insert_if_absent(guard.facing)
    }

    fn find_next_obstacle(&self, guard: &Guard) -> Result<Index2D, Index2D> {
        let Day6 { obstacles_per_column, obstacles_per_row, max_row, max_column, .. } = self;

        let (horizontal, descending, mut exit) = match guard.facing {
            CompassDirection::NORTH => (false, true, Err(Index2D { row: 0, column: guard.position.column })),
            CompassDirection::EAST => (true, false, Err(Index2D { row: guard.position.row, column: *max_column })),
            CompassDirection::SOUTH => (false, false, Err(Index2D { row: *max_row, column: guard.position.column })),
            CompassDirection::WEST => (true, true, Err(Index2D { row: guard.position.row, column: 0 })),
        };
        let (Some(obstacles), reference_value, fixed_value) = (if horizontal {
            (obstacles_per_row.get(guard.position.row), guard.position.column, guard.position.row)
        } else {
            (obstacles_per_column.get(guard.position.column), guard.position.row, guard.position.column)
        }) else { return exit };

        let mut best_distance = usize::MAX;

        for value in obstacles.iter().cloned() {
            let dist = if descending {
                if value >= reference_value {
                    usize::MAX
                } else {
                    reference_value - value
                }
            } else {
                if value <= reference_value {
                    usize::MAX
                } else {
                    value - reference_value
                }
            };

            if dist < best_distance {
                best_distance = dist;
                exit =
                    Ok(if horizontal {
                        Index2D { row: fixed_value, column: value }
                    } else {
                        Index2D { row: value, column: fixed_value }
                    })
            }
        }

        exit
    }
}


fn part1(day: &mut Day6) -> usize {
    let mut continue_walk = true;
    let mut guard = day.initial_guard.clone();
    let mut path_end = day.find_next_obstacle(&guard);

    while continue_walk {
        let terminating_index = match path_end {
            Ok(pos) => {
                pos
            }
            Err(pos) => {
                continue_walk = false;
                pos
            }
        };

        while terminating_index != guard.position + guard.facing {
            day.step(&mut guard);
        }

        if continue_walk {
            guard.turn();
            path_end = day.find_next_obstacle(&guard);
        } else {
            day.step(&mut guard);
        }
    }

    day.visited.len()
}

fn part2(mut day: Day6) -> usize {
    0
}


parsed_day!(parse, part1, part2);