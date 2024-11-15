#[cfg(feature = "gtk_ui")]
mod gtk;
mod console;

use std::fmt::{Arguments};
use std::io::Write;
use std::process::ExitCode;
use clap::ValueEnum;
use crate::Configuration;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, ValueEnum)]
pub enum UIMode {
    #[cfg(feature = "gtk_ui")]
    GTK,
    Console,
    Optimized,
}

impl UIMode {

    pub fn run(self, config: Configuration) -> ExitCode {
        match self {
            #[cfg(feature = "gtk_ui")]
            UIMode::GTK => gtk::gtk_run(config),
            UIMode::Console => console::console_run(config),
            UIMode::Optimized => console::optimized_run(config)
        }
    }
}

pub trait UIOutput<T>  {
    fn info(&mut self, fmt: Arguments<'_>);
    fn critical(&mut self, fmt: Arguments<'_>);
    fn result(&mut self, fmt: Arguments<'_>);
}

pub struct FullUI<T>(pub T);

pub struct OptimizedUI<T>(pub T);

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
    fn info(&mut self, _fmt: Arguments<'_>) {}

    fn critical(&mut self, _fmt: Arguments<'_>) {}

    fn result(&mut self, fmt: Arguments<'_>) {
        write_tagged(&mut self.0, "", fmt)
    }
}