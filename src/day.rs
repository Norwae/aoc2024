use std::error::Error;
use std::fmt::Display;
use std::io::Write;

use crate::ui::UIOutput;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

pub struct Day<T: Write> {
    pub terse: fn(&str, &mut T),
    pub verbose: fn(&str, &mut T),
}

impl<T: Write> Clone for Day<T> {
    fn clone(&self) -> Self {
        Self { terse: self.terse, verbose: self.verbose }
    }
}

#[macro_export] macro_rules! unimplemented_day {
    () => {
        pub const fn register<T: std::io::Write>() -> Option<crate::day::Day<T>> {
            None
        }
    };
}

pub fn parse_and_execute<
    Parse: FnOnce(&str, &mut UI) -> Result<ParseArtifact, ParseError>,
    Part1: FnOnce(&mut ParseArtifact, &mut UI) -> Result1,
    Part2: FnOnce(ParseArtifact, &mut UI) -> Result2,
    ParseArtifact,
    ParseError: Error,
    Result1: Display,
    Result2: Display,
    UI: UIOutput<T>,
    T: Write>(
    parse: Parse, part1: Part1, part2: Part2, input: &str, output: &mut UI,
) -> String {
    let parsed = parse(input, output);
    match parsed {
        Ok(mut parsed) => {
            output.info(format_args!("Parsed input successfully\n"));
            let part1 = part1(&mut parsed, output);
            output.info(format_args!("Completed part 1 calculation: {}\n", &part1));
            let part2 = part2(parsed, output);
            format!("Part1: {}, Part2: {}", part1, part2)
        }
        Err(failed) => {
            output.critical(format_args!("Parsing failed for {}: {}", input, failed));
            "".to_string()
        }
    }
}

pub fn unimplemented_part<T>(_input: &mut T) -> &'static str {
    "UNIMPLEMENTED"
}

#[macro_export] macro_rules! parsed_day {
    ($parse:ident) => {
        use crate::day::unimplemented_part;
        parsed_day!($parse, unimplemented_part, unimplemented_part);
    };
    ($parse:ident, $part1:ident) => {
        use crate::day::unimplemented_part;
        parsed_day!($parse, $part1, unimplemented_part);
    };
    ($parse:expr, $part1:expr, $part2:expr) => {
        use crate::day::parse_and_execute;
        simple_day!(|i, o|parse_and_execute($parse, $part1, $part2, i, &mut o));
    };
 }

#[macro_export] macro_rules! simple_day {
    ($name:ident) => {
        simple_day!(|input, output| {
            $name(input, output)
        })
    };
    (| $n:ident | $body:expr) => {
        simple_day! { |$n, _out| $body }
    };
    (| $n:ident, $out:ident | $body:expr ) => {
        use crate::ui::{UIOutput, FullUI, OptimizedUI};
        use crate::day::Day;
        use std::io::Write;

        fn do_solve<Out: UIOutput<T>, T : Write>($n: &str, mut $out: Out){
            $out.info(format_args!("Started {}\n", module_path!()));
            let result = $body;
            $out.result(format_args!("{}: {result}\n", module_path!()));
        }
        fn solve_terse<T: Write>(input: &str, writer: &mut T) {
            let opt = OptimizedUI(writer);
            do_solve(input, opt)
        }

        fn solve_verbose<T: Write>(input: &str, writer: &mut T) {
            let full = FullUI(writer);
            do_solve(input, full)
        }

        pub const fn register<T: Write>() -> Option<Day<T>> {
            Some(Day {terse: solve_terse::<T>, verbose: solve_verbose::<T> })
        }
    };
 }

pub const fn handlers<T: Write>() -> [Option<Day<T>>; 25] {
    [
        day01::register::<T>(), day02::register::<T>(), day03::register::<T>(),
        day04::register::<T>(), day05::register::<T>(), day06::register::<T>(),
        day07::register::<T>(), day08::register::<T>(), day09::register::<T>(),
        day10::register::<T>(), day11::register::<T>(), day12::register::<T>(),
        day13::register::<T>(), day14::register::<T>(), day15::register::<T>(),
        day16::register::<T>(), day17::register::<T>(), day18::register::<T>(),
        day19::register::<T>(), day20::register::<T>(), day21::register::<T>(),
        day22::register::<T>(), day23::register::<T>(), day24::register::<T>(),
        day25::register::<T>()
    ]
}
