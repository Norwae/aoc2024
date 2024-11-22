#[cfg(feature = "gtk_ui")]
mod gtk;
mod console;

use std::fmt::{Arguments};
use std::io::Write;
use std::process::ExitCode;
use clap::{ValueEnum};
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

pub trait UIWrite : Write {
    fn create<T: Write>(out: &mut T, prefix: &'static str) -> impl UIWrite;
    fn info(&mut self, fmt: Arguments<'_>);
    fn critical(&mut self, fmt: Arguments<'_>);
    fn result(&mut self, fmt: Arguments<'_>);
}

pub struct Verbose<T> {
    out: T,
    prefix: &'static str
}

impl <T: Write> Write for Verbose<T> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.out.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.out.flush()
    }
}

impl <T: Write> UIWrite for Verbose<T> {
    fn create<T2: Write>(out: &mut T2, prefix: &'static str) -> impl UIWrite {
        Verbose { out, prefix }
    }
    fn info(&mut self, fmt: Arguments<'_>) {
        _ = self.write(self.prefix.as_bytes());
        _ = self.write(b" INFO: ");
        _ = self.write_fmt(fmt);
        _ = self.write(b"\n");
        _ = self.flush();
    }

    fn critical(&mut self, fmt: Arguments<'_>) {
        _ = self.write(self.prefix.as_bytes());
        _ = self.write(" CRITICAL: ".as_bytes());
        _ = self.write_fmt(fmt);
        _ = self.write(b"\n");
        _ = self.flush();
    }

    fn result(&mut self, fmt: Arguments<'_>) {
        _ = self.write(self.prefix.as_bytes());
        _ = self.write(" RESULT: ".as_bytes());
        _ = self.write_fmt(fmt);
        _ = self.write(b"\n");
        _ = self.flush();
    }
}

pub struct Terse<T> {
    out: T,
    prefix: &'static str,
    had_output: bool
}
impl <T: Write> Write for Terse<T> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        if self.had_output {
            Ok(buf.len())
        } else {
            self.out.write(buf)
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.had_output = true;
        self.out.flush()
    }
}

impl <T: Write> UIWrite for Terse<T> {
    fn create<T2: Write>(out: &mut T2, prefix: &'static str) -> impl UIWrite {
        Terse { out, prefix, had_output: false }
    }
    fn info(&mut self, _fmt: Arguments<'_>) {
    }

    fn critical(&mut self, _fmt: Arguments<'_>) {
    }

    fn result(&mut self, fmt: Arguments<'_>) {
        _ = self.write(self.prefix.as_bytes());
        _ = self.write(b" ");
        _ = self.write_fmt(fmt);
        _ = self.write(b"\n");
        _ = self.flush()
    }
}