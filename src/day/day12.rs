use crate::collections::{CompassDirection, Index2D, Location2D, Slice2DVisor, Vec2D};
use crate::simple_day;
use std::collections::HashSet;
use std::fmt::format;
use std::mem::swap;
use std::num::NonZero;
use nom::error::dbg_dmp;

simple_day!(|x| solve(x));
fn solve(input: &[u8]) -> String {
    let mut queue = Vec::new();

    let visor = Slice2DVisor::new(input);
    let mut assignment = Vec2D::new_from_flat(
        vec![None::<NonZero<u32>>; visor.rows() * visor.columns()],
        visor.columns(),
    );
    let corner_aspects = |index: Index2D| {
        0
    };
    let mut sum_1 = 0;
    let mut sum_2 = 0;
    let mut next_area_index = 1;
    for idx in assignment.indices() {
        if assignment[idx].is_some() {
            continue;
        }

        let mut area_size = 0;
        let mut area_perimeter = 0;
        let mut area_corner_count = 0;
        let here = visor[idx];
        queue.push(idx);

        while let Some(next) = queue.pop() {
            if assignment[next].is_some() {
                continue;
            }

            area_corner_count += corner_aspects(next);

            assignment[next] = NonZero::<u32>::new(next_area_index);
            area_size += 1;
            for d in CompassDirection::ALL {
                let neighbour = next + d;
                if visor[neighbour] == here {
                    queue.push(neighbour);
                } else {
                    area_perimeter += 1;
                }
            }

        }

        next_area_index += 1;
        sum_1 += area_perimeter * area_size;
        sum_2 += area_corner_count * area_size;
    }

    format!("{sum_1} - {sum_2}")
}
