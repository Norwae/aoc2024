#![feature(never_type)]

mod ui;
mod day;
mod worker;

use std::fs::read;
use clap::Parser;
use std::path::PathBuf;
use std::process::ExitCode;
use crate::ui::{select_ui, UIMode};


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Frontend to load
    #[arg(long, default_value = "gtk", value_enum)]
    ui_mode: UIMode,
    #[arg(long)]
    verbose: bool,
    /// path to read input from
    #[arg(long, value_name = "DIRECTORY", default_value = "./inputfiles")]
    input_path: PathBuf,
    /// days to run (don't specify to run all available)
    #[arg(value_parser = clap::value_parser ! (u8).range(1..=25))]
    run_days: Vec<usize>,
}

#[derive(Clone)]
struct Inputs {
    inputs: [String; 25],
}

impl Inputs {
    fn new(mut input_path: PathBuf) -> Self {
        let mut inputs = [const { String::new() }; 25];
        for i in 0usize..25 {
            input_path.push(format!("{:02}", i + 1));
            if let Ok(loaded_contents) = read(&input_path) {
                let stringified = String::from_utf8(loaded_contents).expect("utf8 input");
                inputs[i] = stringified
            }
            input_path.pop();
        }

        Self { inputs }
    }
}


static ALL_ACTIVE: [usize; 25] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25];

fn main() -> ExitCode {
    let Cli { verbose, input_path, run_days, ui_mode } = Cli::parse();
    let aoc = Inputs::new(input_path);
    let ui = select_ui(ui_mode);
    let preselected = if !run_days.is_empty() {
        run_days.as_slice()
    } else {
        &ALL_ACTIVE
    };
    ui.run(preselected, aoc, verbose)
}