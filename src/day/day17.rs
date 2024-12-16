use crate::collections::{ArrayBag, CompassDirection, Index2D, Vec2D};
use crate::day::parse_graphical_input;
use crate::*;
use nom::IResult;
use std::collections::HashSet;
use std::hash::RandomState;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
struct Node {
    position: Index2D,
    facing: CompassDirection,
}

#[derive(Debug)]
struct Day17 {
    start: Node,
    end: Index2D,
    blocker_map: Vec2D<bool>,
}

fn parse(input: &[u8]) -> Result<Day17, !> {
    let mut temp = Vec::new();
    let mut start = Index2D::IMPLAUSIBLE;
    let mut end = Index2D::IMPLAUSIBLE;
    let Index2D { column, row } = parse_graphical_input(input, |next, idx| match next {
        b'S' => start = idx,
        b'#' => temp.push(idx),
        b'E' => end = idx,
        _ => (),
    });
    let mut blocker_map = Vec2D::filled(false, column + 1, row + 1);
    for idx in temp {
        blocker_map[idx] = true;
    }

    Ok(Day17 {
        start: Node {
            position: start,
            facing: CompassDirection::EAST,
        },
        end,
        blocker_map,
    })
}

fn p1(day: Day17) -> String {
    let (mut iter, score) = pathfinding::directed::astar::astar_bag(
        &day.start,
        |node| {
            let mut buffer = ArrayBag::<_, 3>::default();
            buffer.insert((
                Node {
                    position: node.position,
                    facing: node.facing.turn_right(),
                },
                1000,
            ));
            buffer.insert((
                Node {
                    position: node.position,
                    facing: node.facing.turn_left(),
                },
                1000,
            ));
            let ahead = node.position + node.facing;
            if !day.blocker_map[ahead] {
                buffer.insert((
                    Node {
                        position: ahead,
                        facing: node.facing,
                    },
                    1,
                ));
            }
            buffer
        },
        |node| node.position.manhattan_distance(day.end),
        |node| node.position == day.end,
    ).unwrap();

    let mut best = HashSet::new();
    for path in iter {
        for Node { position, .. } in path {
            best.insert(position);
        }
    }

    format!("Part 1 (score): {score}, Part 2 (distinct best-path squares): {}", best.len())
}

parsed_day!(parse, p1);
