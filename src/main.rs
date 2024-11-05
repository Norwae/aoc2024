#![feature(never_type)]
mod ui;
mod day;

use clap::Parser;
use std::env::args;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Run in headless mode
    #[arg(long, default_value = "false")]
    headless: bool,
    /// always show UI (overrides headless)
    #[arg(long, default_value = "false")]
    force_ui: bool,
    /// path to read input from
    #[arg(long, value_name = "DIRECTORY", default_value = "./inputfiles")]
    input_path: PathBuf,
    /// days to run (don't specify to run all available)
    run_days: Vec<u8>
}


fn main() {
    println!("{:?}", Cli::parse())
}
