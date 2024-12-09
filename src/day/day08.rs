use std::collections::HashSet;
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

fn part1(input: Day8) -> usize {
    dbg!(&input);
    let mut outputs = HashSet::<Index2D>::new();

    for (_, antennae) in input.locations.iter() {
        for antenna1 in antennae.as_ref() {
            for antenna2 in antennae.as_ref() {
                if antenna1 != antenna2 {
                    let distance  = *antenna1 - *antenna2;
                    let antinode = *antenna1 + distance;
                    let index: Index2D = antinode.into();

                    if index.plausible() && index.row <= input.terminus.row && index.column <= input.terminus.column {
                        outputs.insert(index);
                    }
                }
            }
        }
    }

    outputs.len()
}

parsed_day!(parse, part1);