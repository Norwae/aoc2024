use crate::collections::{CompassDirection, Index2D, Vec2D};
use crate::day::parse_graphical_input_raw;
use crate::*;
use std::collections::{VecDeque};

#[derive(Debug)]
struct Maze {
    is_wall_at: Vec2D<bool>,
    cost_to_position: Vec2D<usize>,
    start: Index2D,
    end: Index2D,
}


fn parse(input: &[u8]) -> Result<Maze, !> {
    let mut buffer = Vec::with_capacity(input.len());
    let mut start = Index2D::IMPLAUSIBLE;
    let mut end = Index2D::IMPLAUSIBLE;
    let mut row_length = usize::MAX;
    let mut length = 0;
    parse_graphical_input_raw(input, |next, idx| {
        match next {
            b'\r' => return false,
            b'\n' => {
                row_length = length;
                length = 0;
                return true;
            }
            b'S' => {
                start = idx;
                buffer.push(false);
            }
            b'E' => {
                end = idx;
                buffer.push(false)
            }
            b'#' => buffer.push(true),
            b'.' => buffer.push(false),
            _ => unreachable!(),
        }
        length += 1;
        false
    });
    let is_wall_at = Vec2D::new_from_flat(buffer, row_length);
    let cost_to_position = Vec2D::new_from_flat(vec![usize::MAX; is_wall_at.len()], row_length);

    Ok(Maze {
        is_wall_at,
        start,
        end,
        cost_to_position,
    })
}

fn solve(mut maze: Maze) -> String {
    maze.cost_to_position[maze.end] = 0;
    let mut queue = VecDeque::new();
    queue.push_back(maze.end);

    while let Some(next) = queue.pop_front() {
        let cost = maze.cost_to_position[next];
        for d in CompassDirection::ALL {
            let step = next + d;
            if !maze.is_wall_at[step] && maze.cost_to_position[step] == usize::MAX {
                maze.cost_to_position[step] = cost + 1;
                queue.push_back(step);
            }
        }
    }

    let mut target_buffer = [Index2D::IMPLAUSIBLE; 840];
    let mut cheating_options_long = 0;
    let mut cheating_options_short = 0;

    for source in maze.cost_to_position.indices() {
        let source_cost = maze.cost_to_position[source];
        if source_cost == usize::MAX {
            continue
        }
        let targets = teleport_targets(source, &mut target_buffer, 20);
        for target in targets {
            let jump_distance = source.manhattan_distance(*target);
            if maze.cost_to_position.validate_index(*target) {
                let target_cost = maze.cost_to_position[*target];
                if target_cost != usize::MAX {
                    if target_cost < source_cost {
                        let saved = source_cost - target_cost - jump_distance;
                        if saved >= 100 {
                            cheating_options_long += 1;
                            if jump_distance <= 2 {
                                cheating_options_short += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    format!("Short: {cheating_options_short}, Long: {cheating_options_long}")
}

fn teleport_targets(source: Index2D, target: &mut [Index2D], n: usize) -> &[Index2D] {
    let mut top = 0;
    let mut insert_if_plausible = |idx: Index2D| {
        if idx.plausible() {
            target[top] = idx;
            top += 1;
        }
    };

    for delta_1 in 1usize..=n {
        for d in CompassDirection::ALL {
            let base = source.move_by(delta_1, d);
            insert_if_plausible(base);
            let right = d.turn_right();
            for delta_2 in 1usize..=(n - delta_1) {
                insert_if_plausible(base.move_by(delta_2, right));
            }
        }
    }

    &target[..top]
}

parsed_day!(parse, solve);
