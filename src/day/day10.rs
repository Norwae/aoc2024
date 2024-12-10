use std::collections::HashSet;
use std::mem::swap;
use crate::*;
use crate::collections::{CompassDirection, Index2D, Vec2D};
use crate::day::parse_graphical_input;
use crate::parse_helpers::infallible_parse;

#[derive(Debug)]
struct Tile {
    elevation: u32,
    tops_reachable: HashSet<Index2D>,
    rating: usize
}
fn parse(input: &[u8]) -> Vec2D<Tile> {
    let mut result = Vec::new();

    let Index2D { column: columns, .. } = parse_graphical_input(input, |byte, here| {
        let elevation = (byte - b'0') as u32;
        let mut tops_reachable = HashSet::new();
        let mut rating = 0;

        if elevation == 9 {
            tops_reachable.insert(here);
            rating = 1;
        }

        result.push(Tile { elevation, tops_reachable, rating })
    });

    Vec2D::new_from_flat(result, columns)
}

fn solve_both(mut input: Vec2D<Tile>) -> String {
    for height in (0..9).rev() {
        for idx in input.indices() {
            if input[idx].elevation == height {
                for cd in CompassDirection::ALL {
                    let i2 = idx + cd;
                    if input.validate_index(i2) && input[i2].elevation == height + 1 {
                        let mut tmp = HashSet::new();
                        // swap-and-back trick to prove aliasing safety
                        swap(&mut input[i2].tops_reachable, &mut tmp);
                        input[idx].tops_reachable.extend(&tmp);
                        input[idx].rating += input[i2].rating;
                        swap(&mut input[i2].tops_reachable, &mut tmp);
                    }
                }
            }
        }
    }

    let mut sum_ends = 0;
    let mut sum_ratings = 0;
    for idx in input.indices() {
        let location = &input[idx];
        if location.elevation == 0 {
            sum_ends += location.tops_reachable.len();
            sum_ratings += location.rating;
        }
    }

    format!("{sum_ends} - {sum_ratings}")
}

parsed_day!(infallible_parse(parse), solve_both);