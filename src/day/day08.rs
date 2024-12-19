use std::collections::HashSet;
use fxhash::FxHashSet;
use crate::*;
use crate::collections::{ArrayBag, Index2D, IndexMap, Location2D};
use crate::day::parse_graphical_input;

#[derive(Debug, Default)]
struct Day8 {
    locations: IndexMap<ArrayBag<Location2D, 32>, 128>,
    terminus: Index2D
}
fn parse(input: &[u8]) -> Result<Day8, !> {
    let mut result = Day8::default();
    result.terminus = parse_graphical_input(input, |byte, location| {
        result.locations.get_or_insert_default(byte as usize).insert(location.into())
    });

    Ok(result)
}

fn part1(input: Day8) -> String {
    let plausible = |location: Index2D| location.plausible() &&
        location.row <= input.terminus.row &&
        location.column <= input.terminus.column;

    let mut single_distance_outputs = FxHashSet::default();
    let mut any_distance_outputs = FxHashSet::default();

    for (_, antennae) in input.locations.iter() {
        for antenna1 in antennae.as_ref() {
            for antenna2 in antennae.as_ref() {
                if antenna1 != antenna2 {
                    let distance  = *antenna1 - *antenna2;
                    let single_step = *antenna1 + distance;
                    let index: Index2D = single_step.into();

                    if plausible(index.into()){
                        single_distance_outputs.insert(index);
                    }

                    let mut cursor = *antenna1;

                    while plausible(cursor.into()) {
                        any_distance_outputs.insert(cursor);
                        cursor += distance
                    }
                }
            }
        }
    }

    format!("{} - {}", single_distance_outputs.len(), any_distance_outputs.len())
}

parsed_day!(parse, part1);