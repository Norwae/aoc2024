use crate::*;

parsed_day!(parse, part1, part2);

const STRING_VALUE_PAIRS: [(&'static str, i64, usize); 9] = [
    ("one", -1, 2),
    ("two", -2, 3),
    ("three", -3, 4),
    ("four", -4, 4),
    ("five", -5, 3),
    ("six", -6, 3),
    ("seven", -7, 4),
    ("eight", -8, 4),
    ("nine", -9, 3)
];


fn solve_generic<F: Fn(i64) -> Option<u64>>(input: &Vec<i64>, map: F) -> u64 {
    let mut sum = 0;
    let mut first = u64::MAX;
    let mut last = 0;

    for content in input.into_iter() {
        let content = *content;

        if content == 0 {
            let two_digit_nr = 10 * first + last;
            sum += two_digit_nr;
            first = u64::MAX
        } else if let Some(next) = map(content) {
            if first == u64::MAX {
                first = next;
            }
            last = next;
        }
    }

    sum
}

fn part1(input: &Vec<i64>) -> u64 {
    solve_generic(input, |c| {
        if c >= 0 {
            Some(c as u64)
        } else {
            None
        }
    })
}

fn part2(input: &Vec<i64>) -> u64 {
    solve_generic(input, |c| {
        Some(c.unsigned_abs())
    })
}

fn parse(mut input: &str) -> Result<Vec<i64>, !> {
    let mut result = Vec::new();

    'outer: while !input.is_empty() {
        let first = input.as_bytes()[0];
        if first == b'\n' {
            result.push(0)
        } else if (b'1'..=b'9').contains(&first) {
            result.push((first - b'0') as i64);
        } else {
            for (prefix, value, skip) in &STRING_VALUE_PAIRS {
                if input.starts_with(prefix) {
                    result.push(*value);
                    input = &input[*skip..];
                    continue 'outer
                }
            }
        }

        input = &input[1..]
    }

    Ok(result)
}
