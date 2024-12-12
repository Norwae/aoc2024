use crate::*;

parsed_day!(parse, p1, p2);

struct Span {
    file_id: i32,
    length: u8,
    processed: bool
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
        spans.push(Span { length, file_id, processed: false });

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
    while let Some(mut candidate_index) = (&input[..last]).iter().rposition(|it|it.file_id > 0 && !it.processed) {
        let required_length = input[candidate_index].length;
        input[candidate_index].processed = true;

        if let Some(insert_index) = input[0..candidate_index].iter().position(|it|it.file_id == -1 && it.length >= required_length) {
            if input[insert_index].length != required_length {
                // split hole into two
                input[insert_index].length -= required_length;
                let tmp = Span { file_id: -1, length: required_length, processed: true};
                input.insert(insert_index, tmp);
                candidate_index += 1;
            }
            input.swap(insert_index,candidate_index);
        }
        last = candidate_index;
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