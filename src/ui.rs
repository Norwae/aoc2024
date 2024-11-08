mod gtk;
mod console;

use std::fmt::{Arguments};
use std::io::Write;
use crate::ui::console::{BenchmarkingConsoleUI, SlowConsoleUI};
use std::process::ExitCode;
use clap::ValueEnum;

use crate::ui::gtk::GtkUI;
use super::AdventOfCode;


#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, ValueEnum)]
pub enum UIMode {
    GTK,
    Console,
    Optimized
}

pub trait UIOutput {
    fn info(writer: &mut dyn Write, fmt: Arguments<'_>);
    fn critical(writer: &mut dyn Write, fmt: Arguments<'_>);
    fn result(writer: &mut dyn Write, fmt: Arguments<'_>);
}

pub struct FullUI();
pub struct OptimizedUI();

fn write_wrapped(writer: &mut dyn Write, tag: &str, args: Arguments<'_>) -> Result<(), std::io::Error> {
    writer.write(tag.as_bytes())?;
    writer.write_fmt(args)?;
    writer.flush()
}

fn write_tagged(writer: &mut dyn Write, tag: &str, args: Arguments<'_>) {
    if let Err(err) = write_wrapped(writer, tag, args) {
        eprintln!("Output error: {err}")
    }
}

impl UIOutput for FullUI {
    fn info(writer: &mut dyn Write, fmt: Arguments<'_>) {
        write_tagged(writer, "INFO: ", fmt)
    }

    fn critical(writer: &mut dyn Write, fmt: Arguments<'_>) {
        write_tagged(writer, "CRITICAL: ", fmt)
    }

    fn result(writer: &mut dyn Write, fmt: Arguments<'_>) {
        write_tagged(writer, "RESULT:", fmt)
    }
}

impl UIOutput for OptimizedUI {
    fn info(_writer: &mut dyn Write, _fmt: Arguments<'_>) {
    }

    fn critical(_writer: &mut dyn Write, _fmt: Arguments<'_>) {
    }

    fn result(writer: &mut dyn Write, fmt: Arguments<'_>) {
        write_tagged(writer, "", fmt)
    }
}

pub trait UI {
    fn run(&self, preselected_days: &[u8], aoc: AdventOfCode) -> ExitCode;
}

pub fn select_ui(mode: UIMode) -> Box<dyn UI> {
    match mode  {
        UIMode::GTK => Box::new(GtkUI::new()),
        UIMode::Console => Box::new(SlowConsoleUI()),
        UIMode::Optimized => Box::new(BenchmarkingConsoleUI())
    }
}