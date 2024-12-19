use fxhash::{FxHashMap, FxHashSet};
use crate::*;
use crate::collections::{ArrayBag, CompassDirection, Index2D, IndexMap};
use crate::day::parse_graphical_input;

#[derive(Debug, Default, Clone, Hash, Eq, PartialEq)]
struct GuardPosition {
    facing: CompassDirection,
    position: Index2D,
}

impl GuardPosition {
    fn step(&mut self) {
        self.position += self.facing;
    }

    fn turn(&mut self) {
        self.facing = self.facing.turn_right();
    }
}

#[derive(Debug, Default, Clone)]
struct Day6 {
    initial_guard: GuardPosition,
    obstacles_per_row: IndexMap<ArrayBag<usize, 32>, 256>,
    obstacles_per_column: IndexMap<ArrayBag<usize, 32>, 256>,
    visited: FxHashMap<Index2D, ArrayBag<CompassDirection, 4>>,
    max_row: usize,
    max_column: usize,
}

fn parse(input: &[u8]) -> Result<Day6, !> {
    let mut result = Day6::default();

    let span = parse_graphical_input(input, |byte, position| {
        if byte == b'^' {
            result.initial_guard = GuardPosition { facing: CompassDirection::NORTH, position };
            result.visited.entry(position).or_default().insert(CompassDirection::NORTH);
        } else if byte == b'#' {
            result.obstacles_per_row.get_or_insert_default(position.row).insert(position.column);
            result.obstacles_per_column.get_or_insert_default(position.column).insert(position.row);
        }
    });
    result.max_row = span.row;
    result.max_column = span.column;

    Ok(result)
}

impl Day6 {
    fn step(&mut self, guard: &mut GuardPosition) -> bool {
        guard.step();
        self.visited.entry(guard.position).or_default().insert_if_absent(guard.facing)
    }

    fn find_next_obstacle(&self, guard: &GuardPosition) -> Result<Index2D, Index2D> {
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

fn check_for_circle(input: &Day6) -> bool {
    let mut cursor = input.initial_guard.clone();
    let mut seen = FxHashSet::default();

    while let Ok(obstacle) = input.find_next_obstacle(&cursor) {
        let turning_point = obstacle - cursor.facing;
        cursor.position = turning_point;
        cursor.turn();

        if !seen.insert(cursor.clone()) {
            return true;
        }
    }
    false
}

fn part2(mut day: Day6) -> usize {
    let mut circles = 0;

    for reached in day.visited.keys().cloned() {
        let row = reached.row;
        let column = reached.column;
        if reached != day.initial_guard.position &&
            !day.obstacles_per_row.get_or_insert_default(row).as_ref().contains(&column) {
            day.obstacles_per_row.get_or_insert_default(row).insert(column);
            day.obstacles_per_column.get_or_insert_default(column).insert(row);

            if check_for_circle(&day) {
                circles += 1;
            }
            day.obstacles_per_row.get_or_insert_default(row).remove(&column);
            day.obstacles_per_column.get_or_insert_default(column).remove(&row);
        }
    }

    circles
}


parsed_day!(parse, part1, part2);