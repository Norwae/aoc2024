use crate::*;
use crate::day::day10::KnotHash;
use crate::vec2d::{CompassDirection, Index2D, Vec2D};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct RegionAssignment {
    filled: bool,
    assigned: bool
}

simple_day!(|i|solve(i));
fn solve(input: &str) -> String {
    let input = input.trim();
    let mut used = 0;
    let mut expanded_bit_field = Vec2D::new(128);
    for n in 0..128{
        let mut buffer = [RegionAssignment {filled: false, assigned: true}; 128];
        let hash = KnotHash::hash(format!("{}-{}", input, n).as_bytes());
        used += hash.count_ones();
        expand_i128_to_bool_array(hash, &mut buffer);
        expanded_bit_field.extend_with_row(&buffer);
    }
    
    let mut region_count = 0;
    while let Some(index) = expanded_bit_field.find(&RegionAssignment { filled: true, assigned: false}) {
        region_count += 1;
        mark_region(&mut expanded_bit_field, index)
    }
    
    format!("Part1: {}, Part2: {}", used, region_count)
}

fn mark_region(regions: &mut Vec2D<RegionAssignment>, initial: Index2D) {
    let mut queue = vec![initial];

    while let Some(next) = queue.pop() {
        if regions[next].assigned {
            continue;
        }

        regions[next].assigned = true;

        for d in CompassDirection::ALL {
            let spread_to = next + d;
            if regions.validate_index(spread_to) {
                queue.push(spread_to)
            }
        }
    }
}

fn expand_i128_to_bool_array(mut value: u128, target: &mut [RegionAssignment; 128]) {
    for i in 0..128  {
        if (value & 0x8000_0000_0000_0000_0000_0000_0000_0000) != 0 {
            target[i] = RegionAssignment { filled: true, assigned: false};
        }
        value <<= 1
    }
}