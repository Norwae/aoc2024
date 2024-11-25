use std::error::Error;
use std::fmt::{Display, Formatter};
use std::io::Write;
use nom::IResult;
use crate::ui::UIWrite;

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
    'input,
    'output,
    Parse: FnOnce(&'input str) -> Result<ParseArtifact, ParseError>,
    Part1: FnOnce(&mut ParseArtifact) -> Result1,
    Part2: FnOnce(ParseArtifact) -> Result2,
    ParseArtifact: 'input,
    ParseError: Error +'input,
    Result1: Display,
    Result2: Display,
    UI: UIWrite>(
    parse: Parse, part1: Part1, part2: Part2, input: &'input str, output: &'output mut UI
) -> String {
    let parsed = parse(input);
    match parsed {
        Ok(mut parsed) => {
            output.info(format_args!("Parsed input successfully\n"));
            let part1 = part1(&mut parsed);
            output.info(format_args!("Completed part 1 calculation: {}\n", &part1));
            let part2 = part2(parsed);
            format!("Part1: {}, Part2: {}", part1, part2)
        }
        Err(failed) => {
            output.critical(format_args!("Parsing failed for {}: {}", input, failed));
            format!("ERROR: {}", failed)
        }
    }
}

#[derive(Debug)]
struct SimpleError(String);

impl Display for SimpleError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl Error for SimpleError {}

const fn nom_parsed<'i, ParseResult: 'i, NomFunc: FnOnce(&'i str) -> IResult<&'i str, ParseResult>>(
    nom_handler: NomFunc
) -> impl FnOnce(&'i str) -> Result<ParseResult, SimpleError> {
    |input| {
        match nom_handler(input) {
            Ok((tail, parsed)) if tail.trim().is_empty() => {
                Ok(parsed)
            }
            Ok((rest, _)) => {
                Err(SimpleError(format!("Incomplete parse: {}", rest.trim())))
            }
            Err(e) => {
                Err(SimpleError(format!("{e}")))
            }
        }
    }
}


#[macro_export] macro_rules! parsed_day {
    ($parse:expr) => {
        parsed_day!($parse, |_|"UNIMPLEMENTED Part 1");
    };
    ($parse:expr, $part1:expr) => {
        parsed_day!($parse, $part1, |_|"UNIMPLEMENTED Part 2");
    };
    ($parse:expr, $part1:expr, $part2:expr) => {
        simple_day!(|i, o|crate::day::parse_and_execute($parse, $part1, $part2, i, &mut o));
    };
}

#[macro_export] macro_rules! simple_day {
    ($name:ident) => {
        simple_day!(|input, output| {
            $name(input, &mut output)
        });
    };
    (| $n:ident | $body:expr) => {
        simple_day! { |$n, _out| $body }
    };
    (| $name:ident, $out:ident | $body:expr ) => {
        use crate::ui::{Verbose, Terse, UIWrite};
        use crate::day::Day;
        use std::io::Write;

        pub const fn register<T: Write>() -> Option<Day<T>> {
            fn solve_trampoline<T: Write, UI: UIWrite>($name: &str, writer: &mut T) {
                let mut $out = UI::create(writer, module_path!());

                $out.info(format_args!("Started"));
                let result = $body;
                $out.result(format_args!("{result}"));
            }
            Some(Day {terse: solve_trampoline::<T, Terse<T>>, verbose: solve_trampoline::<T, Verbose<T>> })
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
