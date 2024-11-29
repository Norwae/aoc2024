use std::fmt::Write;
use crate::*;
use crate::vec2d::{CompassDirection, Index2D};


simple_day!(|raw|{
    use CompassDirection::*;
    let input = raw.lines().collect::<Vec<_>>();
    let mut buffer = String::new();
    let mut flow_direction = SOUTH;
    let mut steps = 0;
    let mut position = Index2D {
        row: 0,
        column: input[0].as_bytes().iter().position(|b| *b == b'|').unwrap(),
    };
    let get_with_default = |Index2D {row, column}| {
        if row <= input.len() {
            let row = input[row].as_bytes();
            if column <= row.len() {
                return row[column]
            }
        }

        b' '
    };

    loop {
        let here = get_with_default(position);
        match here {
            b' ' => break,
            b'A'..=b'Z' => buffer.push(here as char),
            b'+' =>
                if flow_direction == NORTH || flow_direction == SOUTH {
                    let west = position + WEST;
                    let at_west = get_with_default(west);
                    if at_west != b' ' {
                        flow_direction = WEST
                    } else {
                        flow_direction = EAST
                    }
                } else {
                    let north = position + NORTH;
                    let at_north = get_with_default(north);
                    if at_north != b' ' {
                        flow_direction = NORTH
                    } else {
                        flow_direction = SOUTH
                    }
                }
            _ => ()
        }
        position = position + flow_direction;
        steps += 1;
    }

    buffer.write_fmt(format_args!(" in {} steps", steps)).unwrap();

    buffer
});