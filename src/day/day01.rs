use std::fmt::format;
use crate::*;

simple_day!(|str|{
    let str = str.trim().as_bytes();
    let half_len = str.len() / 2;
    let mut sum_1 = (if str[0] == str[str.len() - 1] {str[0] - b'0'} else {0}) as u64;
    let mut sum_2 = (if str[0] == str[half_len] {str[0] - b'0'} else {0}) as u64;
    for i in 1..str.len() {
        let add = (str[i] - b'0') as u64;
        if str[i - 1] == str[i] {
            sum_1 += add
        }

        if str[(i + half_len) % str.len()] == str[i] {
            sum_2 += add
        }
    }

    format!("part1: {sum_1}, part2: {sum_2}")
});
