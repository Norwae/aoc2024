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
        pub fn register() -> Option<Box<dyn crate::day::Day>> {
            None
        }
    };
}

pub const REGISTRY: [fn() -> Option<Box<dyn Day>>; 25] = [
    day01::register, day02::register, day03::register, day04::register, day05::register,
    day06::register, day07::register, day08::register, day09::register, day10::register,
    day11::register, day12::register, day13::register, day14::register, day15::register,
    day16::register, day17::register, day18::register, day19::register, day20::register,
    day21::register, day22::register, day23::register, day24::register, day25::register
];

pub trait Day {}