use crate::collections::{ArrayBag, CompassDirection, Index2D, Vec2D};
use crate::day::day20::StepType::AfterCheatFair;
use crate::day::{parse_graphical_input, parse_graphical_input_raw};
use crate::*;
use fxhash::{FxBuildHasher, FxHashSet};
use pathfinding::prelude::{astar, astar_bag, dijkstra, yen, AstarSolution};
use std::collections::HashSet;

#[derive(Debug)]
struct Maze {
    is_wall_at: Vec2D<bool>,
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

    Ok(Maze {
        is_wall_at,
        start,
        end,
    })
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum StepType {
    BeforeCheatFair,
    Teleport,
    AfterCheatFair,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct BannedTeleport(Index2D, Index2D);

fn solve1(maze: Maze) -> usize {
    let (_, fair_cost) = astar(
        &maze.start,
        |idx: &Index2D| {
            let mut array = ArrayBag::<_, 4>::default();
            for dir in CompassDirection::ALL {
                if !maze.is_wall_at[*idx + dir] {
                    array.insert((*idx + dir, 1usize))
                }
            }
            array
        },
        |idx| idx.manhattan_distance(maze.end),
        |idx| *idx == maze.end,
    )
    .unwrap();

    let mut cheat_count_better_than_threshold = 0;
    let threshold = 99;
    let mut banlist = FxHashSet::default();
    while let Some((paths, cheating_cost)) = run_with_cheat(&maze, &banlist) {
        if cheating_cost < fair_cost - threshold {
            for path in paths {
                cheat_count_better_than_threshold += 1;
                let teleport_idx = path
                    .iter()
                    .position(|it| it.0 == StepType::Teleport)
                    .unwrap();
                banlist.insert(BannedTeleport(
                    path[teleport_idx - 1].1,
                    path[teleport_idx].1,
                ));
            }
            dbg!(cheat_count_better_than_threshold, cheating_cost, fair_cost);
        } else {
            break;
        }
    }
    cheat_count_better_than_threshold
}

fn teleport_targets(source: Index2D, target: &mut [Index2D]) -> &[Index2D] {
    let mut top = 0;
    let mut insert_if_plausible = |idx: Index2D| {
        if idx.plausible() {
            target[top] = idx;
            top += 1;
        }
    };

    for delta_1 in 0usize..=20 {
        for delta_2 in 0..=(20 - delta_1) {
            insert_if_plausible(
                source
                    .move_by(delta_1, CompassDirection::NORTH)
                    .move_by(delta_2, CompassDirection::EAST),
            );
            insert_if_plausible(
                source
                    .move_by(delta_1, CompassDirection::NORTH)
                    .move_by(delta_2, CompassDirection::WEST),
            );
            insert_if_plausible(
                source
                    .move_by(delta_1, CompassDirection::SOUTH)
                    .move_by(delta_2, CompassDirection::EAST),
            );
            insert_if_plausible(
                source
                    .move_by(delta_1, CompassDirection::SOUTH)
                    .move_by(delta_2, CompassDirection::WEST),
            );
        }
    }

    &target[..top]
}

fn run_with_cheat(
    maze: &Maze,
    banned_cheats: &HashSet<BannedTeleport, FxBuildHasher>,
) -> Option<(AstarSolution<(StepType, Index2D)>, usize)> {
    let mut buffer = [Index2D::IMPLAUSIBLE; 1600];
    astar_bag(
        &(StepType::BeforeCheatFair, maze.start),
        |(state, from)| {
            let is_before_cheat = *state == StepType::BeforeCheatFair;
            let next_type = if is_before_cheat {
                StepType::BeforeCheatFair
            } else {
                AfterCheatFair
            };
            let mut bag = ArrayBag::<_, 1700>::default();
            for dir in CompassDirection::ALL {
                if !maze.is_wall_at[*from + dir] {
                    bag.insert(((next_type, *from + dir), 1usize))
                }
            }

            if is_before_cheat {
                bag.extend(
                    teleport_targets(*from, &mut buffer)
                        .into_iter()
                        .cloned()
                        .filter(|it| maze.is_wall_at.validate_index(*it) && !maze.is_wall_at[*it])
                        .filter(|to| !banned_cheats.contains(&BannedTeleport(*from, *to)))
                        .map(|idx| ((StepType::Teleport, idx), from.manhattan_distance(idx))),
                );
            }

            bag
        },
        |(_, idx)| idx.manhattan_distance(maze.end),
        |(_, idx)| *idx == maze.end,
    )
}

parsed_day!(parse, solve1);
