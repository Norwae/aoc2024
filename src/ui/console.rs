use std::io::{stdout, Stdout};
use std::process::ExitCode;
use std::time::{Duration, Instant};
use crate::Configuration;
use crate::day::{Day, handlers};
use crate::timed::{bin_duration, time_span, ALL_WORK};
use crate::worker::parallelize_ordered;


pub fn console_run(config: Configuration) -> ExitCode {
    static HANDLERS: [Option<Day<Stdout>>; 25] = handlers::<Stdout>();
    for day in config.active_days() {
        let index = (*day - 1) as usize;
        if let Some(solution) = &HANDLERS[index] {
            let handler = if config.verbose {
                solution.verbose
            } else {
                solution.terse
            };
            handler(&config.load_input(*day), &mut stdout());
        }
    }

    ExitCode::SUCCESS
}

fn execute_day_handler(day: u8, day_handler_function: fn(&[u8], &mut Vec<u8>), input: Vec<u8>) -> OptimizedOutput {
    let mut output_buffer = Vec::new();
    let (_, timing) = time_span(|| day_handler_function(&input, &mut output_buffer));
    OptimizedOutput {
        day,
        timing,
        output_buffer,
    }
}

struct OptimizedOutput {
    day: u8,
    timing: Duration,
    output_buffer: Vec<u8>,
}

pub fn optimized_run(config: Configuration) -> ExitCode {
    static HANDLERS: [Option<Day<Vec<u8>>>; 25] = handlers::<Vec<u8>>();
    let clock_start = Instant::now();
    let tasks = config.active_days().into_iter().filter_map(|day| {
        let day = *day;
        let index = (day - 1) as usize;
        if let Some(handler) = &HANDLERS[index] {
            let handler = if config.verbose {
                handler.verbose
            } else {
                handler.terse
            };
            let input = config.load_input(day);
            Some(move || execute_day_handler(day, handler, input))
        } else {
            None
        }
    });
    let mut overall_output = String::new();
    let mut day_eval_timings = String::new();
    let results = parallelize_ordered(tasks);
    for OptimizedOutput {day, timing, output_buffer} in results {
        overall_output += &String::from_utf8(output_buffer).expect("Valid utf8");
        day_eval_timings += &format!("Day {day}: {timing:?}\n")
    }

    let clock_duration = Instant::now() - clock_start;
    let total_duration = bin_duration(ALL_WORK);

    println!("Overall run complete.
Wall time: {clock_duration:?}
Sum Task CPU time: {total_duration:?}

Output:
{overall_output}

Day evaluation times:
{day_eval_timings}");

    ExitCode::SUCCESS
}

