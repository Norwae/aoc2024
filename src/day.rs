use std::io::Write;
use crate::ui::{FullUI, OptimizedUI};

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
pub struct Day<T : Write> {
    pub handler: fn(&str, &mut T)
}

#[macro_export] macro_rules! unimplemented_day {
    () => {
        pub fn register<Out: crate::ui::UIOutput<T>, T: std::io::Write>() -> Option<crate::day::Day<T>> {
            None
        }
    };
}

#[macro_export] macro_rules! parsed_day {
     ($parse:ident, $part1:ident, $part2:ident) => {
         simple_day! { |input, out| {
             match parse(input) {
                 Ok(parsed) => {
                     Out::info(out, format_args!("Parsed input successfully\n"));
                     let part1 = $part1(&parsed);
                     Out::info(out, format_args!("Completed part 1 calculation: {}\n", &part1));
                     let part2 = $part2(&parsed);
                     format!("Part1: {}, Part2: {}", part1, part2)
                 },
                 Err(failed) => {
                     Out::critical(out, format_args!("Parsing failed for {}: {}", input, failed));
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
         use crate::ui::UIOutput;
         use crate::day::Day;
         use std::io::Write;

         fn do_solve<Out: UIOutput<T>, T : Write>($n: &str, $out: &mut T){
             Out::info($out, format_args!("Started {}\n", module_path!()));
             let result = $body;
             Out::result($out, format_args!("{}: {result}\n", module_path!()));
         }
         pub fn register<Out: UIOutput<T>, T: Write>() -> Option<Day<T>> {
             Some(Day { handler: do_solve::<Out, T> })
         }
     };
 }

pub fn handlers<T: Write>(optimized: bool) -> [fn() -> Option<Day<T>>; 25] {
    if optimized {
        [
            day01::register::<OptimizedUI, T>, day02::register::<OptimizedUI, T>, day03::register::<OptimizedUI, T>,
            day04::register::<OptimizedUI, T>, day05::register::<OptimizedUI, T>, day06::register::<OptimizedUI, T>,
            day07::register::<OptimizedUI, T>, day08::register::<OptimizedUI, T>, day09::register::<OptimizedUI, T>,
            day10::register::<OptimizedUI, T>, day11::register::<OptimizedUI, T>, day12::register::<OptimizedUI, T>,
            day13::register::<OptimizedUI, T>, day14::register::<OptimizedUI, T>, day15::register::<OptimizedUI, T>,
            day16::register::<OptimizedUI, T>, day17::register::<OptimizedUI, T>, day18::register::<OptimizedUI, T>,
            day19::register::<OptimizedUI, T>, day20::register::<OptimizedUI, T>, day21::register::<OptimizedUI, T>,
            day22::register::<OptimizedUI, T>, day23::register::<OptimizedUI, T>, day24::register::<OptimizedUI, T>,
            day25::register::<OptimizedUI, T>
        ]
    } else {
        [
            day01::register::<FullUI, T>, day02::register::<FullUI, T>, day03::register::<FullUI, T>,
            day04::register::<FullUI, T>, day05::register::<FullUI, T>, day06::register::<FullUI, T>,
            day07::register::<FullUI, T>, day08::register::<FullUI, T>, day09::register::<FullUI, T>,
            day10::register::<FullUI, T>, day11::register::<FullUI, T>, day12::register::<FullUI, T>,
            day13::register::<FullUI, T>, day14::register::<FullUI, T>, day15::register::<FullUI, T>,
            day16::register::<FullUI, T>, day17::register::<FullUI, T>, day18::register::<FullUI, T>,
            day19::register::<FullUI, T>, day20::register::<FullUI, T>, day21::register::<FullUI, T>,
            day22::register::<FullUI, T>, day23::register::<FullUI, T>, day24::register::<FullUI, T>,
            day25::register::<FullUI, T>
        ]
    }

}
