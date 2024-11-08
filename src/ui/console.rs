use std::io::{stdout, Stdout};
use std::process::ExitCode;
use std::sync::mpsc::{channel, Sender};
use std::time::{Duration, Instant};
use crate::day::handlers;
use crate::Inputs;
use crate::ui::UI;

pub struct SlowConsoleUI();
pub struct BenchmarkingConsoleUI();

impl UI for SlowConsoleUI {
    fn run(&self, preselected_days: &[u8], aoc: Inputs, verbose: bool) -> ExitCode {
        let handlers = handlers::<Stdout>(!verbose);
        for day in preselected_days {
            let index = *day as usize - 1;
            if let Some(day) = handlers[index]() {
                (day.handler)(&aoc.inputs[index],  &mut stdout());
            }
        }

        ExitCode::SUCCESS
    }
}

fn execute_day_handler(function: fn(&str, &mut Vec<u8>), input: String, out: Sender<OptimizedOutput>) {
    let mut output_buffer = Vec::new();
    let start = Instant::now();
    function(&input, &mut output_buffer);
    let timing = Instant::now() - start;
    out.send(OptimizedOutput {
        timing, output_buffer
    }).unwrap()
}

struct OptimizedOutput {
    output_buffer: Vec<u8>,
    timing: Duration
}

impl UI for BenchmarkingConsoleUI {
    fn run(&self, preselected_days: &[u8], aoc: Inputs, verbose: bool) -> ExitCode {
        let handler_functions = handlers::<Vec<u8>>(!verbose).map(|f|f());
        let mut expected_answers = 0;
        let (sender, receive) = channel();
        let pool = threadpool::Builder::new()
            .thread_name("Worker".to_string())
            .build();
        let clock_start = Instant::now();
        for day in  preselected_days {
            let index = *day as usize -1;
            if let Some(callback) = handler_functions[index].clone() {
                let sender= sender.clone();
                let input = aoc.inputs[index].clone();
                expected_answers += 1;
                pool.execute(move ||{
                    execute_day_handler(callback.handler, input, sender)
                })
            }
        }
        drop(sender);

        let mut total_duration = Duration::ZERO;
        let mut overall_output = String::new();
        for _ in 0..expected_answers {
            let OptimizedOutput { timing, output_buffer } = receive.recv().expect("Received answer");
            total_duration += timing;
            overall_output += &String::from_utf8(output_buffer).expect("Valid utf8");
        }

        let clock_duration = Instant::now() - clock_start;

        println!("Overall run complete. Wall time: {clock_duration:?}, Sum Task CPU time: {total_duration:?}, Output: \n{overall_output}");

        ExitCode::SUCCESS
    }
}