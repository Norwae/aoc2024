use crate::*;

simple_day!(|string, _out|{
    let string = string.trim().as_bytes();
    let half_len = string.len() / 2;
    let mut sum_1 = (if string[0] == string[string.len() - 1] {string[0] - b'0'} else {0}) as u64;
    let mut sum_2 = (if string[0] == string[half_len] {string[0] - b'0'} else {0}) as u64;
    for i in 1..string.len() {
        let add = (string[i] - b'0') as u64;
        if string[i - 1] == string[i] {
            sum_1 += add
        }

        if string[(i + half_len) % string.len()] == string[i] {
            sum_2 += add
        }
    }

    format!("part1: {sum_1}, part2: {sum_2}")
});
