mod gtk;
mod console;

use std::fmt::{Arguments};
use std::io::Write;
use crate::ui::console::{BenchmarkingConsoleUI, SlowConsoleUI};
use std::process::ExitCode;
use clap::ValueEnum;

use crate::ui::gtk::GtkUI;
use super::Inputs;


#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, ValueEnum)]
pub enum UIMode {
    GTK,
    Console,
    Optimized
}

pub trait UIOutput<T: Write> {
    fn info(writer: &mut T, fmt: Arguments<'_>);
    fn critical(writer: &mut T, fmt: Arguments<'_>);
    fn result(writer: &mut T, fmt: Arguments<'_>);
}

pub struct FullUI();
pub struct OptimizedUI();

fn write_wrapped<T: Write>(writer: &mut T, tag: &str, args: Arguments<'_>) -> Result<(), std::io::Error> {
    writer.write(tag.as_bytes())?;
    writer.write_fmt(args)?;
    writer.flush()
}

fn write_tagged<T: Write>(writer: &mut T, tag: &str, args: Arguments<'_>) {
    if let Err(err) = write_wrapped(writer, tag, args) {
        eprintln!("Output error: {err}")
    }
}

impl <T: Write> UIOutput<T> for FullUI {
    fn info(writer: &mut T, fmt: Arguments<'_>) {
        write_tagged(writer, "INFO: ", fmt)
    }

    fn critical(writer: &mut T, fmt: Arguments<'_>) {
        write_tagged(writer, "CRITICAL: ", fmt)
    }

    fn result(writer: &mut T, fmt: Arguments<'_>) {
        write_tagged(writer, "RESULT:", fmt)
    }
}

impl <T: Write> UIOutput<T> for OptimizedUI {
    fn info(_writer: &mut T, _fmt: Arguments<'_>) {
    }

    fn critical(_writer: &mut T, _fmt: Arguments<'_>) {
    }

    fn result(writer: &mut T, fmt: Arguments<'_>) {
        write_tagged(writer, "", fmt)
    }
}

pub trait UI {
    fn run(&self, preselected_days: &[u8], inputs: Inputs, verbose: bool) -> ExitCode;
}

pub fn select_ui(mode: UIMode) -> Box<dyn UI> {
    match mode  {
        UIMode::GTK => Box::new(GtkUI),
        UIMode::Console => Box::new(SlowConsoleUI),
        UIMode::Optimized => Box::new(BenchmarkingConsoleUI)
    }
}