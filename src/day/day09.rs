use std::mem::swap;
use crate::*;
use crate::ui::UIWrite;

simple_day!(solve);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Mode {
    FILE,
    SPACE
}

fn solve(input: &[u8], ui: &mut impl UIWrite) -> usize {
    ui.info(format_args!("Got {} input bytes", input.len()));
    let mut memory = Vec::new();
    let mut file_id = 0;
    let mut mode = Mode::FILE;
    for next_byte in input {
        let next_span = *next_byte - b'0';
        for _ in 0..next_span {
            memory.push(if mode == Mode::FILE { file_id } else { - 1 })
        }

        if mode == Mode::FILE {
            mode = Mode::SPACE;
        } else {
            mode = Mode::FILE;
            file_id += 1;
        }
    }
    let mut front = 0;
    let mut back = memory.len() - 1;

    while front < back {
        if memory[front] != -1 {
            front += 1;
        } else if memory[back] == -1 {
            back -= 1;
        } else {
            memory.swap(front, back);
        }
    }

    checksum(memory.as_slice())
}

fn checksum(memory: &[i32]) -> usize {
    memory.iter().enumerate().map(|(n, v)|{
        if *v > 0 {
            n * (*v as usize)
        } else {
            0
        }
    }).sum()
}