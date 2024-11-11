use std::io::Write;

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

#[derive(Clone)]
pub struct Day<T: Write> {
    pub terse: fn(&str, &mut T),
    pub verbose: fn(&str, &mut T),
}

#[macro_export] macro_rules! unimplemented_day {
    () => {
        pub fn register<T: std::io::Write>() -> Option<crate::day::Day<T>> {
            None
        }
    };
}


#[macro_export] macro_rules! memoized_day {
    ($parse:ident) => {
        fn unimplemented_part_1<T>(_input: &T) -> (&'static str, ()) {
            ("UNIMPLEMENTED", ())
        }

        fn unimplemented_part_2<T>(_input: &T, _memo: ()) -> &'static str {
            "UNIMPLEMENTED"
        }
        memoized_day!($parse, unimplemented_part_1, unimplemented_part_2);
    };
    ($parse:ident, $part1:ident) => {
        fn unimplemented_part<T, Memo>(_input: &T, _memo: Memo) -> &'static str {
            "UNIMPLEMENTED"
        }
        memoized_day!($parse, $part1, unimplemented_part);
    };
    ($parse:ident, $part1:ident, $part2:ident) => {
         simple_day! { |input, out| {
             match parse(input) {
                 Ok(parsed) => {
                     out.info(format_args!("Parsed input successfully\n"));
                     let (part1, memo) = $part1(&parsed);
                     out.info(format_args!("Completed part 1 calculation: {} (memo {:?})\n", &part1, &memo));
                     let part2 = $part2(&parsed, memo);
                     format!("Part1: {}, Part2: {}", part1, part2)
                 },
                 Err(failed) => {
                     out.critical(format_args!("Parsing failed for {}: {}", input, failed));
                     "".to_string()
                 }
             }
         }}
    };
 }

#[macro_export] macro_rules! parsed_day {
    ($parse:ident) => {
        fn unimplemented_part<T>(_input: &T) -> &'static str {
            "UNIMPLEMENTED"
        }
        parsed_day!($parse, unimplemented_part, unimplemented_part);
    };
    ($parse:ident, $part1:ident) => {
        fn unimplemented_part<T>(_input: &T) -> &'static str {
            "UNIMPLEMENTED"
        }
        parsed_day!($parse, $part1, unimplemented_part);
    };
    ($parse:ident, $part1:ident, $part2:ident) => {
         simple_day! { |input, out| {
             match parse(input) {
                 Ok(parsed) => {
                     out.info(format_args!("Parsed input successfully\n"));
                     let part1 = $part1(&parsed);
                     out.info(format_args!("Completed part 1 calculation: {}\n", &part1));
                     let part2 = $part2(&parsed);
                     format!("Part1: {}, Part2: {}", part1, part2)
                 },
                 Err(failed) => {
                     out.critical(format_args!("Parsing failed for {}: {}", input, failed));
                     "".to_string()
                 }
             }
         }}
    };
 }

#[macro_export] macro_rules! simple_day {
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

         pub fn register<T: Write>() -> Option<Day<T>> {
             Some(Day {
                 terse: solve_terse::<T>,
                 verbose: solve_verbose::<T>
             })
         }
     };
 }

pub fn handlers<T: Write>() -> [fn() -> Option<Day<T>>; 25] {
    [
        day01::register::<T>, day02::register::<T>, day03::register::<T>,
        day04::register::<T>, day05::register::<T>, day06::register::<T>,
        day07::register::<T>, day08::register::<T>, day09::register::<T>,
        day10::register::<T>, day11::register::<T>, day12::register::<T>,
        day13::register::<T>, day14::register::<T>, day15::register::<T>,
        day16::register::<T>, day17::register::<T>, day18::register::<T>,
        day19::register::<T>, day20::register::<T>, day21::register::<T>,
        day22::register::<T>, day23::register::<T>, day24::register::<T>,
        day25::register::<T>
    ]
}
