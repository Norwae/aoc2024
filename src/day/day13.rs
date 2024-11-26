use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::combinator::map;
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::tuple;
use crate::*;
use crate::day::nom_parsed;
use crate::parse_helpers::parse_unsigned_nr;

struct Scanner {
    depth: usize,
    range: usize,
}

impl Scanner {
    fn lane_at_timestamp(&self, ts: usize) -> usize {
        let steps_in_pass = self.periodicity();
        let mut position_in_pass = ts % steps_in_pass;
        if position_in_pass > self.range || (position_in_pass == self.range && self.range % 2 == 1){
            position_in_pass = steps_in_pass - position_in_pass;
        }
        position_in_pass
    }

    fn periodicity(&self) -> usize {
        2 * self.range - 2
    }
}

fn parse_scanner(input: &str) -> IResult<&str, Scanner> {
    map(tuple((
        parse_unsigned_nr,
        tag(": "),
        parse_unsigned_nr
    )), |(depth, _, range)| Scanner { depth, range })(input)
}

parsed_day!(nom_parsed(separated_list1(line_ending, parse_scanner)), p1, p2);

fn p1(scanners: &mut Vec<Scanner>) -> usize {
    let mut severity = 0;
    scanners.sort_by_key(|s|s.depth);
    for scanner in scanners {
        if scanner.lane_at_timestamp(scanner.depth) == 0 {
            severity += scanner.depth * scanner.range
        }
    }

    severity
}


fn p2(scanners: Vec<Scanner>) -> usize {
    let mut delay = 0;
    let mut delay_increment = 1;

    'wait: loop {
        for scanner in &scanners {
            if scanner.lane_at_timestamp(delay + scanner.depth) == 0 {
                delay += delay_increment;
                continue 'wait
            }

            if scanner.range == 2 {
                delay_increment = 2;
            }
        }

        return delay
    }

}