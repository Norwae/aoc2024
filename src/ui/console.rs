use std::fmt::format;
use std::io::{stdout, Stdout};
use std::process::ExitCode;
use std::sync::mpsc::{channel, Sender};
use std::time::{Duration, Instant};
use crate::Configuration;
use crate::day::{Day, handlers};
use crate::worker::run_on_worker;


pub fn console_run(config: Configuration) -> ExitCode {
    static HANDLERS: [Option<Day<Stdout>>; 25] = handlers::<Stdout>();
    for day in &config.run_days {
        let index = *day - 1;
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

fn execute_day_handler(day: usize, function: fn(&str, &mut Vec<u8>), input: String, out: Sender<OptimizedOutput>) {
    let mut output_buffer = Vec::new();
    let start = Instant::now();
    function(&input, &mut output_buffer);
    let timing = Instant::now() - start;
    out.send(OptimizedOutput {
        day,
        timing,
        output_buffer,
    }).unwrap()
}

struct OptimizedOutput {
    day: usize,
    output_buffer: Vec<u8>,
    timing: Duration,
}

pub fn optimized_run(config: Configuration) -> ExitCode {
    static HANDLERS: [Option<Day<Vec<u8>>>; 25] = handlers::<Vec<u8>>();
    let clock_start = Instant::now();
    let mut expected_answers = 0;
    let (sender, receive) = channel();
    for day in config.active_days() {
        let day = *day;
        let index = day - 1;
        if let Some(callbacks) = &HANDLERS[index] {
            let sender = sender.clone();
            let handler = if config.verbose {
                callbacks.verbose
            } else {
                callbacks.terse
            };
            let input = config.load_input(index + 1);
            expected_answers += 1;
            run_on_worker(move || {
                execute_day_handler(day, handler, input, sender)
            })
        }
    }
    drop(sender);

    let mut total_duration = Duration::ZERO;
    let mut overall_output = String::new();
    let mut day_eval_timings = String::new();
    for _ in 0..expected_answers {
        let OptimizedOutput { day, timing, output_buffer } = receive.recv().expect("Received answer");
        total_duration += timing;
        overall_output += &String::from_utf8(output_buffer).expect("Valid utf8");
        day_eval_timings += &format!("Day {day}: {timing:?}")
    }

    let clock_duration = Instant::now() - clock_start;

    println!("Overall run complete.
Wall time: {clock_duration:?}
Sum Task CPU time: {total_duration:?}

Output:
{overall_output}

Day evaluation times:
{day_eval_timings}");

    ExitCode::SUCCESS
}
