use nom::character::complete::u32;
use crate::*;
use crate::vec2d::Index2D;

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

struct Visor<'a> {
    bytes: &'a [u8],
    newline_at: usize,
}

impl<'a> Visor<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        let newline_at = bytes.iter().position(|it| *it == b'\n').unwrap();

        Self { bytes, newline_at }
    }

    fn columns(&self) -> usize {
        self.newline_at
    }

    fn rows(&self) -> usize {
        self.bytes.len() / (self.newline_at + 1)
    }

    fn at(&self, row: i32, column: i32) -> u8 {
        if row < 0 || column < 0 || column as usize >= self.columns() || row as usize >= self.rows() {
            b'!'
        } else {
            let offset = (self.newline_at + 1) * row as usize + column as usize;
            self.bytes[offset]
        }
    }
}

fn solve_1(visor: &Visor) -> usize {
    let mut bytes = [0u8; 4];
    let mut found = 0;

    for row in 0..visor.rows() as i32 {
        for column in 0..visor.columns() as i32 {
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

fn solve_2(visor: Visor) -> usize {
    let mut bytes = [0u8; 5];
    let mut found = 0;

    for row in 0..visor.rows() as i32 {
        'col: for column in 0..visor.columns() as i32 {
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

parsed_day!(|str|Ok::<_, !>(Visor::new(str)), |i|solve_1(i), solve_2);