mod gtk;

use std::process::ExitCode;
use clap::ValueEnum;

use crate::ui::gtk::GtkUI;
use super::AdventOfCode;


#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, ValueEnum)]
pub enum UIMode {
    GTK,
    Console,
    OptimizedHeadless
}
pub trait UI {
    fn run(&self, preselected_days: Vec<u8>, aoc: AdventOfCode) -> ExitCode;
}

pub fn select_ui(mode: UIMode) -> Box<dyn UI> {
    match mode  {
        UIMode::GTK => Box::new(GtkUI::new()),
        _ => panic!("UIMode {mode:?} not yet implemented")
    }
}