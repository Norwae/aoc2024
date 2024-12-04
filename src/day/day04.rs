use nom::character::complete::u32;
use crate::*;
use crate::vec2d::Index2D;

const XMAS: u32 = 0x584D4153;

struct Visor<'a, F> {
    bytes: &'a [u8],
    newline_at: usize,
    out_of_bounds: F,
}

impl<'a, F: Fn(i32, i32, usize, &[u8]) -> u8> Visor<'a, F> {
    fn new(bytes: &'a [u8], out_of_bounds: F) -> Self {
        let newline_at = bytes.iter().position(|it| *it == b'\n').unwrap();

        Self { bytes, out_of_bounds, newline_at }
    }

    fn columns(&self) -> usize {
        self.newline_at
    }

    fn rows(&self) -> usize {
        self.bytes.len() / (self.newline_at + 1)
    }

    fn at(&self, row: i32, column: i32) -> u8 {
        if row < 0 || column < 0 || column as usize >= self.columns() || row as usize >= self.rows() {
            (&self.out_of_bounds)(row, column , self.newline_at, self.bytes)
        } else {
            let offset = (self.newline_at + 1) * row as usize + column as usize;
            self.bytes[offset]
        }
    }
}

fn solve(input: &str) -> String {
    let cutting_visor = Visor::new(input.as_bytes(), |_, _ , _, _| b'!');
    let mut bytes = [0u8; 4];
    let mut found = 0;

    for row in 0..cutting_visor.rows() as i32 {
        for column in 0..cutting_visor.columns() as i32 {
            bytes[0] = cutting_visor.at(row, column);
            bytes[1] = cutting_visor.at(row, column + 1);
            bytes[2] = cutting_visor.at(row, column + 2);
            bytes[3] = cutting_visor.at(row, column + 3);

            if u32::from_be_bytes(bytes) == XMAS || u32::from_le_bytes(bytes) == XMAS {
                found += 1;
            }

            bytes[1] = cutting_visor.at(row + 1, column + 1);
            bytes[2] = cutting_visor.at(row + 2, column + 2);
            bytes[3] = cutting_visor.at(row + 3, column + 3);

            if u32::from_be_bytes(bytes) == XMAS || u32::from_le_bytes(bytes) == XMAS {
                found += 1;
            }


            bytes[1] = cutting_visor.at(row + 1, column);
            bytes[2] = cutting_visor.at(row + 2, column);
            bytes[3] = cutting_visor.at(row + 3, column);

            if u32::from_be_bytes(bytes) == XMAS || u32::from_le_bytes(bytes) == XMAS {
                found += 1;
            }

            bytes[1] = cutting_visor.at(row + 1, column - 1);
            bytes[2] = cutting_visor.at(row + 2, column - 2);
            bytes[3] = cutting_visor.at(row + 3, column - 3);

            if u32::from_be_bytes(bytes) == XMAS || u32::from_le_bytes(bytes) == XMAS {
                found += 1;
            }
        }
    }

    format!("Part 1: {found}")
}

simple_day!(|i|solve(i));