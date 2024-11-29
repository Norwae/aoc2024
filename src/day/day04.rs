use std::collections::HashSet;
use std::io::Write;
use crate::simple_day;

simple_day!(solve);

fn solve<T: Write>(input: &str, _out: &mut T) -> String {
    let mut valid_part_1 = 0;
    let mut valid_part_2 = 0;
    let mut p1_hashes = HashSet::new();
    let mut p2_hashes = HashSet::new();

    'line: for line in input.lines() {
        p1_hashes.clear();

        for word in line.split(" ") {
            if !p1_hashes.insert(word) {
                continue 'line
            }
        }
        valid_part_1 += 1;

        p2_hashes.clear();
        for word in &p1_hashes {
            let mut bytes = word.as_bytes().to_vec();
            bytes.sort();

            if !p2_hashes.insert(bytes) {
                continue 'line
            }
        }
        valid_part_2 += 1;
    }

    format!("{valid_part_1} {valid_part_2}")
}
