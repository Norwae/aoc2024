#![feature(never_type)]

mod ui;
mod day;
mod worker;
mod vec2d;

mod timed;

mod parse_helpers;

use std::fs::read;
use clap::Parser;
use std::path::PathBuf;
use std::process::ExitCode;
use crate::ui::{UIMode};


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Configuration {
    /// Frontend to load
    #[arg(long, default_value = "console", value_enum)]
    ui_mode: UIMode,
    #[arg(long)]
    verbose: bool,
    /// path to read input from
    #[arg(long, value_name = "DIRECTORY", default_value = "./inputfiles")]
    input_path: PathBuf,
    /// days to run (don't specify to run all available)
    #[arg(value_parser = clap::value_parser ! (u8).range(1..=25))]
    run_days: Vec<u8>,
}

impl Configuration {
    fn run(self) -> ExitCode {
        self.ui_mode.run(self)
    }

    fn active_days(&self) -> &[u8]{
        static ALL_ACTIVE: [u8; 25] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25];

        if self.run_days.is_empty() {
            &ALL_ACTIVE
        } else {
            self.run_days.as_slice()
        }
    }

    fn load_input(&self, day: u8) -> String {
        let mut path = self.input_path.clone();
        path.push(format!("{:02}", day));

        if let Ok(loaded_contents) = read(&path) {
            let mut stringified = String::from_utf8(loaded_contents).expect("utf8 input");
            if !stringified.ends_with("\n") {
                stringified.push('\n');
            }
            stringified
        } else {
            "".to_string()
        }
    }
}

fn main() -> ExitCode {
    worker::warm_up();
    let cfg = Configuration::parse();
    cfg.run()
}
