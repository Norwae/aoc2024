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

#[macro_export] macro_rules! unimplemented_day {
    () => {
        pub fn register<Out: crate::ui::UIOutput>() -> Option<fn (&str, &mut dyn std::io::Write)> {
            None
        }
    };
}

#[macro_export] macro_rules! simple_day {
    (| $n:ident | $body:expr) => {
        simple_day! { |$n, _out| $body }
    };
     (| $n:ident, $out:ident | $body:expr ) => {
         use crate::ui::UIOutput;

         fn do_solve<Out: UIOutput>($n: &str, $out: &mut dyn std::io::Write){
             Out::info($out, format_args!("Started {}\n", module_path!()));
             let result = $body;
             Out::result($out, format_args!("{}: {result}\n", module_path!()));
         }
         pub fn register<T: UIOutput>() -> Option<fn (&str, &mut dyn std::io::Write)> {
             Some(do_solve::<T>)
         }
     };
 }

pub fn handlers(optimized: bool) -> [fn() -> Option<fn(&str, &mut dyn Write)>; 25] {
    if optimized {
        [
            day01::register::<OptimizedUI>, day02::register::<OptimizedUI>, day03::register::<OptimizedUI>,
            day04::register::<OptimizedUI>, day05::register::<OptimizedUI>, day06::register::<OptimizedUI>,
            day07::register::<OptimizedUI>, day08::register::<OptimizedUI>, day09::register::<OptimizedUI>,
            day10::register::<OptimizedUI>, day11::register::<OptimizedUI>, day12::register::<OptimizedUI>,
            day13::register::<OptimizedUI>, day14::register::<OptimizedUI>, day15::register::<OptimizedUI>,
            day16::register::<OptimizedUI>, day17::register::<OptimizedUI>, day18::register::<OptimizedUI>,
            day19::register::<OptimizedUI>, day20::register::<OptimizedUI>, day21::register::<OptimizedUI>,
            day22::register::<OptimizedUI>, day23::register::<OptimizedUI>, day24::register::<OptimizedUI>,
            day25::register::<OptimizedUI>
        ]
    } else {
        [
            day01::register::<FullUI>, day02::register::<FullUI>, day03::register::<FullUI>,
            day04::register::<FullUI>, day05::register::<FullUI>, day06::register::<FullUI>,
            day07::register::<FullUI>, day08::register::<FullUI>, day09::register::<FullUI>,
            day10::register::<FullUI>, day11::register::<FullUI>, day12::register::<FullUI>,
            day13::register::<FullUI>, day14::register::<FullUI>, day15::register::<FullUI>,
            day16::register::<FullUI>, day17::register::<FullUI>, day18::register::<FullUI>,
            day19::register::<FullUI>, day20::register::<FullUI>, day21::register::<FullUI>,
            day22::register::<FullUI>, day23::register::<FullUI>, day24::register::<FullUI>,
            day25::register::<FullUI>
        ]
    }

}
