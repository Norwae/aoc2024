use std::io::stdout;
use std::process::ExitCode;
use crate::AdventOfCode;
use crate::ui::UI;

pub struct ConsoleUI();

impl UI for ConsoleUI {
    fn run(&self, preselected_days: &[u8], aoc: AdventOfCode) -> ExitCode {
        for day in preselected_days {
            let index = *day as usize - 1;
            if let Some(day) = aoc.days[index] {
                day(&aoc.inputs[index],  &mut stdout());
            }
        }

        ExitCode::SUCCESS
    }
}