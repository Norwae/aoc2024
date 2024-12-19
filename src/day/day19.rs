use std::collections::HashMap;
use std::rc::Rc;
use crate::ui::UIWrite;
use crate::*;
use nom::bytes::complete::{tag, take_while, take_while1};
use nom::character::complete::line_ending;
use nom::multi::{many1, separated_list1};
use nom::IResult;

#[derive(Debug)]
struct OnsenPatterns<'a> {
    available_patterns: Rc<Vec<&'a str>>,
    requested: Rc<Vec<&'a str>>,
    construction_cache: HashMap<&'a str, usize>
}

impl <'a> OnsenPatterns<'a> {
    fn construction_counts(&mut self, target: &'a str) -> usize {
        if let Some(cached) = self.construction_cache.get(target) {
            return *cached;
        }

        let mut count = 0;
        for pattern in self.available_patterns.clone().iter() {
            let pattern = *pattern;
            if target.starts_with(pattern) {
                count += self.construction_counts(&target[pattern.len()..]);
            }
        }

        self.construction_cache.insert(target, count);

        count
    }

    fn do_solve(&mut self) -> (usize, usize) {
        self.construction_cache.insert("", 1);
        let mut possible = 0;
        let mut permutations = 0;
        for target in self.requested.clone().iter() {
            let permutations_found = self.construction_counts(target);
            permutations += permutations_found;
            possible += if permutations_found > 0 { 1 } else { 0 };
        }

        (possible, permutations)
    }
}

fn parse_pattern(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| "wubrg".contains(c))(input)
}

fn parse(input: &str) -> IResult<&str, OnsenPatterns> {
    let (rest, available_patterns) = separated_list1(tag(", "), parse_pattern)(input)?;
    // skip 2 separators
    let (rest, requested) = separated_list1(line_ending, parse_pattern)(&rest[2..])?;

    Ok((
        rest,
        OnsenPatterns {
            available_patterns: Rc::new(available_patterns),
            requested: Rc::new(requested),
            construction_cache: HashMap::new()
        },
    ))
}

fn solve(input: &[u8], _out: &mut impl UIWrite) -> String {
    let input = String::from_utf8_lossy(input);
    let mut problem= parse(input.as_ref()).unwrap().1;

    let (solvable, permutations) = problem.do_solve();
    format!("Solvable: {solvable}, permutations {permutations}")
}

simple_day!(solve);
