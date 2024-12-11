use std::collections::HashMap;
use std::mem::swap;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::IResult;
use nom::multi::separated_list1;
use crate::*;
use crate::day::nom_parsed_bytes;
use crate::parse_helpers::parse_unsigned_nr_bytes;

#[derive(Debug)]
struct Day11 {
    frequencies: HashMap<u64, usize>
}

impl Day11 {
    fn apply_blink(&mut self) {
        let mut temp = HashMap::new();
        swap(&mut temp, &mut self.frequencies);

        for (k, v) in temp {
            if k == 0 {
                *self.frequencies.entry(1).or_default() += v;
            } else {
                let number_of_digits = 1 + k.ilog10();
                if number_of_digits % 2 == 0 {
                    let divisor = 10_u64.pow(number_of_digits / 2);
                    *self.frequencies.entry(k / divisor).or_default() += v;
                    *self.frequencies.entry(k % divisor).or_default() += v;
                } else {
                    *self.frequencies.entry(k * 2024).or_default() += v;
                }
            }
        }
    }
}
fn parse(input: &[u8]) -> IResult<&[u8], Day11> {
    map(separated_list1(tag(b" "), parse_unsigned_nr_bytes::<u64>),
        |input|{
            let mut frequencies = HashMap::new();
            for nr in input {
                *frequencies.entry(nr).or_default() += 1;
            }
            Day11 { frequencies }
        }
    )(input)
}

parsed_day!(nom_parsed_bytes(parse), | v|{
    for _ in 0..25 { v.apply_blink() }
    v.frequencies.values().sum::<usize>()
}, |mut v| {
    for _ in 0..50 { v.apply_blink() }
    v.frequencies.values().sum::<usize>()
});