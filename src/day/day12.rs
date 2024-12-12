use crate::*;
use crate::collections::{CompassDirection, Location2D, Vec2D, Visor};

simple_day!(|input|{
    let mut queue= Vec::new();
    let visor = Visor::new(input);
    let mut visited = Vec2D::new_from_flat(vec![false; visor.rows() * visor.columns()], visor.columns());
    let mut sum = 0;
    for idx in visited.indices(){
        if !visited[idx]{
            let mut area_size = 0;
            let mut area_perimeter = 0;
            let here = visor[idx];
            queue.push(idx);

            while let Some(next) = queue.pop(){
                if visor[next] == here && !visited[next]{
                    visited[next] = true;
                    area_size += 1;
                    for d in CompassDirection::ALL {
                        if visor[next + d] == here {
                            queue.push(next + d);
                        } else {
                            area_perimeter += 1;
                        }
                    }
                }
            }

            sum += area_perimeter * area_size
        }
    }

    sum
});