use crate::*;
use crate::collections::Slice2DVisor;

const XMAS: &'static [u8;4] = b"XMAS";
const SAMX: &'static [u8;4] = b"SAMX";

const X_MAS_PATTERNS: [&'static [u8; 5]; 4] = [
    /* M S
        A
       M S 
     */
    b"AMSMS",
    /* M M
        A
       S S 
     */
    b"AMMSS",
    /* S M
        A
       S M 
     */
    b"ASMSM",
    /* S S
        A
       M M 
     */
    b"ASSMM",
];
fn solve_1(visor: &Slice2DVisor) -> usize {
    let mut bytes = [0u8; 4];
    let mut found = 0;

    for row in 0..visor.rows() as i64 {
        for column in 0..visor.columns() as i64 {
            bytes[0] = visor.at(row, column);
            if bytes[0] != b'X' && bytes[0] != b'S' {
                continue
            }

            bytes[1] = visor.at(row, column + 1);
            bytes[2] = visor.at(row, column + 2);
            bytes[3] = visor.at(row, column + 3);

            if &bytes == XMAS || &bytes == SAMX {
                found += 1;
            }

            bytes[1] = visor.at(row + 1, column + 1);
            bytes[2] = visor.at(row + 2, column + 2);
            bytes[3] = visor.at(row + 3, column + 3);

            if &bytes == XMAS || &bytes == SAMX {
                found += 1;
            }


            bytes[1] = visor.at(row + 1, column);
            bytes[2] = visor.at(row + 2, column);
            bytes[3] = visor.at(row + 3, column);

            if &bytes == XMAS || &bytes == SAMX {
                found += 1;
            }

            bytes[1] = visor.at(row + 1, column - 1);
            bytes[2] = visor.at(row + 2, column - 2);
            bytes[3] = visor.at(row + 3, column - 3);

            if &bytes == XMAS || &bytes == SAMX {
                found += 1;
            }
        }
    }

    found
}

fn solve_2(visor: Slice2DVisor) -> usize {
    let mut bytes = [0u8; 5];
    let mut found = 0;

    for row in 0..visor.rows() as i64 {
        'col: for column in 0..visor.columns() as i64 {
            bytes[0] = visor.at(row + 1, column + 1);
            if bytes[0] != b'A'  {
                continue
            }

            bytes[1] = visor.at(row, column);
            bytes[2] = visor.at(row, column + 2);
            bytes[3] = visor.at(row + 2, column);
            bytes[4] = visor.at(row + 2, column + 2);
            for pattern in X_MAS_PATTERNS {
                if &bytes == pattern {
                    found += 1;
                    continue 'col;
                }
            }
        }
    }

    found
}

parsed_day!(|str|Ok::<_, !>(Slice2DVisor::new(str)), |i|solve_1(i), solve_2);