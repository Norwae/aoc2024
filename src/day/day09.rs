use std::mem::swap;
use crate::*;

parsed_day!(parse, p1, p2);

struct Span {
    file_id: i32,
    length: u8,
    moved: bool
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
        spans.push(Span { length, file_id, moved: false });

        if mode == Mode::FILE {
            mode = Mode::SPACE;
        } else {
            mode = Mode::FILE;
            next_id += 1;
        }
    }

    Ok(spans)
}

fn p1(input: &mut Vec<Span>) -> usize {
    let mut front = 0;
    let mut memory: Vec<_> = input.iter().flat_map(|Span{length, file_id, ..}|{
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

fn p2(mut input: Vec<Span>) -> usize {
    let mut last = input.len();
    while let Some(candidate_index) = (&input[..last]).iter().rposition(|it|it.file_id > 0 && !it.moved) {
        let required_length = input[candidate_index].length;
        last = candidate_index;
        if let Some(insert_index) = input[0..candidate_index].iter().position(|it|it.file_id == -1 && it.length >= required_length) {
            let mut tmp = input.remove(candidate_index);
            tmp.moved = true;
            input[insert_index].length -= required_length;
            if input[insert_index].length == 0 {
                swap(&mut tmp, &mut input[insert_index]);
            } else {
                input.insert(insert_index, tmp);
            }
        }
    }
    let memory: Vec<_> = input.iter().flat_map(|Span{length, file_id, ..}|{
        (0..*length).map(move |_|*file_id)
    }).collect();

    checksum(&memory)
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