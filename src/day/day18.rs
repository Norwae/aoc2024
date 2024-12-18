use crate::collections::{ArrayBag, CompassDirection, Index2D, Vec2D};
use crate::day::nom_parsed_bytes;
use crate::parse_helpers::parse_unsigned_nr_bytes;
use crate::*;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;
use pathfinding::prelude::astar;
use nom::combinator::map;

struct Input {
    map: Vec2D<usize>,
    list: Vec<Index2D>
}
fn parse(input: &[u8]) -> IResult<&[u8], Input> {
    let (rest, list) = separated_list1(
        line_ending,
        map(separated_pair(
            parse_unsigned_nr_bytes::<usize>,
            tag(","),
            parse_unsigned_nr_bytes::<usize>,
        ), |(x, y)|Index2D { row: y, column: x }),
    )(input)?;
    let mut map = Vec2D::new_from_flat(vec![usize::MAX; (TARGET.column + 1) * (TARGET.row + 1)], TARGET.column + 1);

    for (ts, idx) in list.iter().enumerate() {
        map[*idx] = ts;
    }

    Ok((rest, Input { map, list }))
}

const TARGET: Index2D = Index2D { row: 70, column: 70 };

struct State {
    input: Input,
    threshold: usize,
}

impl State {
    fn new(input: Input) -> Self {
        Self { input, threshold: 1024 }
    }

    fn criticial_position(&self) -> usize {
        self.threshold - 1
    }

    fn success(&self, idx: &Index2D) -> bool {
        *idx == TARGET
    }

    fn heuristic(&self, idx: &Index2D) -> usize {
        idx.manhattan_distance(TARGET)
    }

    fn update_threshold(&mut self, path: &Vec<Index2D>) {
        let critical_index = path.iter().map(|idx|self.input.map[*idx]).min().unwrap();
        self.threshold = critical_index + 1;
    }

    fn successors(&self, idx: &Index2D) -> impl IntoIterator<Item=(Index2D, usize)> {
        let idx = *idx;
        let mut bag = ArrayBag::<(Index2D, usize), 4>::default();
        for direction in CompassDirection::ALL {
            let stepped = idx + direction;
            if stepped.plausible()
                && stepped.row <= TARGET.row
                && stepped.column <= TARGET.column
            {
                let value = self.input.map[stepped];
                if value == usize::MAX || value >= self.threshold {
                    bag.insert((stepped, 1));
                }
            }
        }
        bag
    }
}

fn solve(input: Input) -> String {
    let mut state = State::new(input);
    let (mut path, cost_1) = astar(
        &Index2D::ZERO,
        |idx|state.successors(idx),
        |idx|state.heuristic(idx),
        |idx|state.success(idx),
    ).unwrap();


    loop {
        state.update_threshold(&path);
        let next_path = astar(
            &Index2D::ZERO,
            |idx|state.successors(idx),
            |idx|state.heuristic(idx),
            |idx|state.success(idx),
        );

        match next_path {
            Some((path2, _)) => {
                path = path2;
            }
            None => {
                break;
            }
        }
    }

    format!("{}, blocker at ({},{})", cost_1, state.input.list[state.criticial_position()].column, state.input.list[state.criticial_position()].row)
}

parsed_day!(nom_parsed_bytes(parse), solve);
