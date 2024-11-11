use std::cell::RefCell;
use std::io::{stdout, Stdout, Write};
use std::process::ExitCode;
use std::rc::Rc;
use std::sync::mpsc::{channel, Sender};
use std::time::{Duration, Instant};
use crate::day::handlers;
use crate::Inputs;
use crate::ui::UI;
use crate::worker::run_on_worker;

pub struct SlowConsoleUI;

pub struct BenchmarkingConsoleUI;

impl UI for SlowConsoleUI {
    fn run(&self, preselected_days: &[u8], aoc: Inputs, verbose: bool) -> ExitCode {
        let handlers = handlers::<Stdout>();
        for day in preselected_days {
            let index = *day as usize - 1;
            if let Some(day) = handlers[index]() {
                let handler = if verbose {
                    day.verbose
                } else {
                    day.terse
                };
                handler(&aoc.inputs[index], stdout());
            }
        }

        ExitCode::SUCCESS
    }
}

fn execute_day_handler(function:  fn(&str, BenchmarkBuffer), input: String, out: Sender<OptimizedOutput>) {
    let output_buffer = Rc::new(RefCell::new(Vec::new()));
    let start = Instant::now();
    function(&input, BenchmarkBuffer(output_buffer.clone()));
    let timing = Instant::now() - start;
    let output_buffer = Rc::into_inner(output_buffer).expect("Sole owner");
    let output_buffer = output_buffer.into_inner();
    out.send(OptimizedOutput {
        timing,
        output_buffer
    }).unwrap()
}

struct OptimizedOutput {
    output_buffer: Vec<u8>,
    timing: Duration,
}

struct BenchmarkBuffer(Rc<RefCell<Vec<u8>>>);
impl Write for BenchmarkBuffer {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.borrow_mut().write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl UI for BenchmarkingConsoleUI {
    fn run(&self, preselected_days: &[u8], aoc: Inputs, verbose: bool) -> ExitCode {
        let handler_functions = handlers::<BenchmarkBuffer>().map(|f| f());
        let mut expected_answers = 0;
        let (sender, receive) = channel();
        let clock_start = Instant::now();
        for day in preselected_days {
            let index = *day as usize - 1;
            if let Some(callbacks) = &handler_functions[index] {
                let sender = sender.clone();
                let handler = if verbose {
                    callbacks.verbose
                } else {
                    callbacks.terse
                };
                let input = aoc.inputs[index].clone();
                expected_answers += 1;
                run_on_worker(move || {
                    execute_day_handler(handler, input, sender)
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