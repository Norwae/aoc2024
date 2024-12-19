use crate::ui::UIWrite;
use crate::*;
use fxhash::FxHashMap;
use nom::bytes::complete::{tag, take_while1};
use nom::character::complete::line_ending;
use nom::multi::separated_list1;
use nom::IResult;
use std::cell::RefCell;

#[derive(Debug)]
struct OnsenPatterns<'a> {
    available_patterns: Vec<&'a str>,
    requested: Vec<&'a str>,
    construction_cache: RefCell<FxHashMap<&'a str, usize>>,
}

impl<'a> OnsenPatterns<'a> {
    fn construction_counts(&self, target: &'a str) -> usize {
        if let Some(cached) = self.construction_cache.borrow().get(target) {
            return *cached;
        }

        let mut count = 0;
        for pattern in self.available_patterns.iter() {
            let pattern = *pattern;
            if target.starts_with(pattern) {
                count += self.construction_counts(&target[pattern.len()..]);
            }
        }

        self.construction_cache.borrow_mut().insert(target, count);

        count
    }

    fn do_solve(&mut self) -> (usize, usize) {
        self.construction_cache.borrow_mut().insert("", 1);
        let mut possible = 0;
        let mut permutations = 0;
        for target in self.requested.iter() {
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
    let (rest, mut available_patterns) = separated_list1(tag(", "), parse_pattern)(input)?;
    // skip 2 separators
    let (rest, mut requested) = separated_list1(line_ending, parse_pattern)(&rest[2..])?;

    available_patterns.sort_by_key(|it| it.len());
    requested.sort_by_key(|it| it.len());

    Ok((
        rest,
        OnsenPatterns {
            available_patterns,
            requested,
            construction_cache: RefCell::default(),
        },
    ))
}

fn solve(input: &[u8], _out: &mut impl UIWrite) -> String {
    let input = String::from_utf8_lossy(input);
    let mut problem = parse(input.as_ref()).unwrap().1;

    let (solvable, permutations) = problem.do_solve();
    format!("Solvable: {solvable}, permutations {permutations}")
}

simple_day!(solve);
