use std::collections::HashSet;
use std::io::Write;
use nom::AsBytes;
use crate::day::Day;

pub const fn register<T: Write>() -> Option<Day<T>> {
    Some(Day {
        verbose: solve,
        terse: solve,
    })
}

fn solve<W: Write>(input: &str, out: &mut W) {
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

    out.write_fmt(format_args!("Day 4: {valid_part_1} {valid_part_2}")).unwrap()
}
