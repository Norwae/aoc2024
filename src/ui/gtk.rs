use std::cell::RefCell;
use std::fmt::format;
use std::process::ExitCode;
use std::rc::Rc;
use gtk4::{Application, ApplicationWindow, Grid, Box as LayoutBox, Orientation, CheckButton, Button, Label, Widget};
use gtk4::glib;
use gtk4::glib::*;
use gtk4::prelude::*;

use crate::AdventOfCode;
use crate::ui::UI;

pub struct GtkUI {
    app: Application,
}

fn build_day_selector_widget(idx: usize, initial_state: bool) -> CheckButton {
    CheckButton::builder()
        .label(format!("Day {}", idx + 1))
        .active(initial_state)
        .build()
}

fn build_day_selector_grid(activations: Rc<RefCell<[bool; 25]>>, availabilities: &[bool]) -> Grid {
    let initial_activations = activations.borrow();
    let grid = Grid::builder()
        .column_homogeneous(true)
        .column_spacing(2)
        .build();
    for idx in 0usize..=24 {
        let selector_widget = if !availabilities[idx] {
            Label::builder().use_markup(true)
                .label(format!("<i>Day {}</i>", idx + 1))
                .build()
                .upcast::<Widget>()
        } else {
            let button = build_day_selector_widget(idx, initial_activations[idx]);

            button.connect_toggled(clone!(
                #[weak] activations,
                move |button|{
                    activations.borrow_mut()[idx] = button.is_active();
                }
            ));
            button.upcast()
        };

        grid.attach(&selector_widget, (idx as i32) % 5, (idx as i32) / 5, 1, 1)
    }

    grid
}

fn build_big_run_button(activations: Rc<RefCell<[bool; 25]>>, state: Rc<AdventOfCode>) -> Button {
    let button = Button::builder()
        .label("Run selected")
        .build();

    button.connect_clicked(clone!(
        #[weak] activations,
        #[weak] state,
        move |b| {
            for idx in 0..25 {
                if activations.borrow()[idx] {
                    println!("Would run {idx}: {}", state.days[idx].is_some())
                }
            }
        })
    );

    button
}

fn build_ui(app: &Application, activations: Rc<RefCell<[bool; 25]>>, state: Rc<AdventOfCode>) {
    let layout = LayoutBox::builder()
        .orientation(Orientation::Vertical)
        .spacing(3)
        .build();

    let available_days = state.days.iter().map(Option::is_some).collect::<Vec<_>>();
    layout.append(&build_day_selector_grid(activations.clone(), &available_days));
    layout.append(&build_big_run_button(activations, state));

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Advent of Code, 2024 Edition")
        .child(&layout)
        .build();

    window.present()
}

impl GtkUI {
    pub fn new() -> GtkUI {
        let app = Application::builder()
            .application_id("codecentric.aoc.AoC2024")
            .build();
        let days_active = Rc::new(RefCell::new([false; 25]));

        Self {
            app
        }
    }
}

impl UI for GtkUI {
    fn run(&self, preselected_days: Vec<u8>, aoc: AdventOfCode) -> ExitCode {
        let aoc = Rc::new(aoc);
        let days_active = Rc::new(RefCell::new(
            if preselected_days.is_empty() {
                [true; 25]
            } else {
                let mut days = [false; 25];
                for day in preselected_days {
                    days[(day as usize) - 1] = true;
                }

                days
            }
        ));
        self.app.connect_activate(clone!(
            #[weak] aoc,
            #[weak] days_active,
            move |app| {
                build_ui(app, days_active, aoc)
            })
        );

        if self.app.run_with_args::<&str>(&[]).value() == 0 {
            ExitCode::SUCCESS
        } else {
            ExitCode::FAILURE
        }
    }
}