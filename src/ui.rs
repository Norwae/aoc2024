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
    Optimized,
}

/*
Why not defer the type arg to an instance? Because we can't do the simple type inference
from _just_ the Write implication in the various register implementations. We do a sneaky type
switch there and lifting this into the signature of Day would mean we need to know which output style
is actually connected. Could be solved by an enum (which will make it a runtime if) but we want to
avoid having a conditional branch if possible. Is this psychotic? Maybe.
*/

pub trait UIOutput<T: Write> {
    fn new(writer: T) -> Self;
    fn info(&mut self, fmt: Arguments<'_>);
    fn critical(&mut self, fmt: Arguments<'_>);
    fn result(&mut self, fmt: Arguments<'_>);
}

pub struct FullUI<T>(T);

pub struct OptimizedUI<T>(T);

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

impl<T: Write> UIOutput<T> for FullUI<T> {
    fn new(writer: T) -> Self {
        Self(writer)
    }
    fn info(&mut self, fmt: Arguments<'_>) {
        write_tagged(&mut self.0, "INFO: ", fmt)
    }

    fn critical(&mut self, fmt: Arguments<'_>) {
        write_tagged(&mut self.0, "CRITICAL: ", fmt)
    }

    fn result(&mut self, fmt: Arguments<'_>) {
        write_tagged(&mut self.0, "RESULT:", fmt)
    }
}

impl<T: Write> UIOutput<T> for OptimizedUI<T> {
    fn new(writer: T) -> Self {
        Self(writer)
    }
    fn info(&mut self, _fmt: Arguments<'_>) {}

    fn critical(&mut self, _fmt: Arguments<'_>) {}

    fn result(&mut self, fmt: Arguments<'_>) {
        write_tagged(&mut self.0, "", fmt)
    }
}

pub trait UI {
    fn run(&self, preselected_days: &[u8], inputs: Inputs, verbose: bool) -> ExitCode;
}

pub fn select_ui(mode: UIMode) -> Box<dyn UI> {
    match mode {
        UIMode::GTK => Box::new(GtkUI),
        UIMode::Console => Box::new(SlowConsoleUI),
        UIMode::Optimized => Box::new(BenchmarkingConsoleUI)
    }
}