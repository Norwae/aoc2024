use std::mem::swap;
use crate::*;
use crate::ui::UIWrite;

parsed_day!(parse, p1);

struct Span {
    file_id: i32,
    length: u8,
}
fn parse(input: &[u8]) -> Result<Vec<Span>, !> {
    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    enum Mode {
        FILE,
        SPACE,
    }

    let mut spans = Vec::new();
    let mut next_id = 0;
    let mut mode = Mode::FILE;
    for next_byte in input {
        let length = *next_byte - b'0';
        let file_id = if mode == Mode::FILE { next_id } else { -1 };
        spans.push(Span { length, file_id });

        if mode == Mode::FILE {
            mode = Mode::SPACE;
        } else {
            mode = Mode::FILE;
            next_id += 1;
        }
    }

    Ok(spans)
}

fn p1(input: Vec<Span>) -> usize {
    let mut front = 0;
    let mut memory: Vec<_> = input.iter().flat_map(|Span{length, file_id}|{
        (0..*length).map(move |_|*file_id)
    }).collect();
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
    memory.iter().enumerate().map(|(n, v)| {
        if *v > 0 {
            n * (*v as usize)
        } else {
            0
        }
    }).sum()
}