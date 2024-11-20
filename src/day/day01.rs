use crate::*;


parsed_day!(|input,_|parse(input), |input, _|{
    solve_generic(input, |c|c)
}, |input, _|{
    solve_generic(input.as_slice(), i64::abs)
});

const STRING_VALUE_PAIRS: [(&'static str, i64, usize); 9] = [
    ("one", -1, 2),
    ("two", -2, 2),
    ("three", -3, 4),
    ("four", -4, 4),
    ("five", -5, 3),
    ("six", -6, 3),
    ("seven", -7, 4),
    ("eight", -8, 4),
    ("nine", -9, 3)
];


fn solve_generic(input: &[i64], process: impl Fn(i64) -> i64) -> i64 {
    let mut sum = 0;
    let mut first = i64::MAX;
    let mut last = 0;

    for content in input.as_ref().into_iter() {
        let content = *content;

        if content == 0 {
            if first != i64::MAX {
                let two_digit_nr = first * 10 + last;
                sum += two_digit_nr;
                first = i64::MAX
            }
        } else {
            let content = process(content);
            if content > 0 {
                if first == i64::MAX {
                    first = content;
                }
                last = content;
            }
        }
    }

    sum
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
                    continue 'outer;
                }
            }
        }

        input = &input[1..]
    }

    Ok(result)
}
