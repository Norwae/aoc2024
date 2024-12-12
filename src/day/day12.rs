use crate::collections::{CompassDirection, Slice2DVisor, Vec2D};
use crate::{parsed_day, simple_day};

parsed_day!(|x|Ok::<_, !>(Slice2DVisor::new(x)), solve);
fn solve(visor: Slice2DVisor) -> String {
    let mut queue = Vec::new();

    let mut assigned = Vec2D::new_from_flat(
        vec![false; visor.rows() * visor.columns()],
        visor.columns(),
    );
    let mut sum_1 = 0;
    let mut sum_2 = 0;
    for idx in assigned.indices() {
        if assigned[idx] {
            continue;
        }

        let mut area_size = 0;
        let mut area_perimeter = 0;
        let mut area_corner_count = 0;
        let here = visor[idx];
        queue.push(idx);

        while let Some(next) = queue.pop() {
            let mut detect_corner = |d1, d2| {
                // outer corner pattern
                // ..
                // #.
                if visor[next + d1] != here && visor[next + d2] != here {
                    area_corner_count += 1;
                }

                // inner corner pattern
                // #.
                // ##
                if visor[next + d1] == here && visor[next + d2] == here && visor[next + d1 + d2] != here {
                    area_corner_count += 1;
                }
            };

            if assigned[next] {
                continue;
            }
            assigned[next] = true;

            detect_corner(CompassDirection::NORTH, CompassDirection::WEST);
            detect_corner(CompassDirection::NORTH, CompassDirection::EAST);
            detect_corner(CompassDirection::SOUTH, CompassDirection::WEST);
            detect_corner(CompassDirection::SOUTH, CompassDirection::EAST);

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

        sum_1 += area_perimeter * area_size;
        sum_2 += area_corner_count * area_size;
    }

    format!("{sum_1} - {sum_2}")
}
