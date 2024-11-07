use std::cell::RefCell;
use std::fmt::format;
use std::io::{ErrorKind, stdout, Write};
use std::mem::swap;
use std::process::ExitCode;
use std::rc::Rc;
use std::sync::RwLock;
use gtk4::{Application, ApplicationWindow, Grid, Box as LayoutBox, Orientation, CheckButton, Button, Label, Widget, StackSidebar, StackSwitcher, Stack, Separator, Text, TextView, TextBuffer, TextIter};
use gtk4::glib;
use gtk4::glib::*;
use gtk4::prelude::*;

use std::sync::mpsc;
use std::sync::mpsc::{channel, Sender};

use crate::AdventOfCode;
use crate::day::Day;
use crate::ui::UI;

pub struct GtkUI;

fn build_day_selector_widget(idx: usize, model: Rc<RefCell<UIModel>>) -> Widget {
    match &model.borrow().days[idx] {
        Some(day) => {
            let button = CheckButton::builder()
                .label(format!("Day {}", idx + 1))
                .active(day.active)
                .build();

            button.connect_toggled(clone!(
                #[weak] model,
                move |button|{
                    model.borrow_mut().days[idx].as_mut().unwrap().active = button.is_active();
                }
            ));

            button.upcast()
        }
        _ =>
            Label::builder().use_markup(true)
                .label(format!("<i>Day {}</i>", idx + 1))
                .build()
                .upcast()
    }
}

fn build_input_editor(n: usize, day: &UIDay, model: Rc<RefCell<UIModel>>) -> TextView {
    let buffer = TextBuffer::builder()
        .text(&day.input)
        .enable_undo(true)
        .build();

    let text = TextView::builder()
        .width_request(500)
        .buffer(&buffer)
        .monospace(true)
        .build();


    buffer.connect_changed(clone!(
        #[weak] model,
        move |txt| {
            let text = txt.text(&txt.start_iter(), &txt.end_iter(), false);
            model.borrow_mut().days[n].as_mut().unwrap().input = text.to_string();
        })
    );

    text
}

fn build_input_stack_pages(model: Rc<RefCell<UIModel>>) -> LayoutBox {
    let layout = LayoutBox::new(Orientation::Horizontal, 2);
    let stack = Stack::new();
    let separator = Separator::new(Orientation::Vertical);
    let selector = StackSidebar::builder()
        .stack(&stack)
        .width_request(100)
        .height_request(400)
        .build();

    for (n, d) in model.borrow().days.iter().enumerate() {
        if let Some(day) = d {
            let input_editor = build_input_editor(n, day, model.clone());
            let name = format!("day_{}", n);
            let label = format!("Day {}", n + 1);
            stack.add_titled(&input_editor, Some(&name), &label);
        }
    }

    layout.append(&selector);
    layout.append(&separator);
    layout.append(&stack);

    layout
}

fn build_day_selector_grid(model: Rc<RefCell<UIModel>>) -> Grid {
    let grid = Grid::builder()
        .column_homogeneous(true)
        .column_spacing(2)
        .build();
    for idx in 0usize..=24 {
        let selector_widget = build_day_selector_widget(idx, model.clone());
        grid.attach(&selector_widget, (idx as i32) % 5, (idx as i32) / 5, 1, 1)
    }

    grid
}

fn perform_run(model: Rc<RefCell<UIModel>>, send: Sender<String>) {
    struct WrapSender(Sender<String>, Vec<u8>);
    impl Write for WrapSender {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            dbg!(&buf);
            self.1.write(buf)
        }

        fn flush(&mut self) -> std::io::Result<()> {
            let mut captured = Vec::new();
            swap(&mut captured, &mut self.1);
            let string = String::from_utf8(captured).expect("UTF8");
            self.0.send(string).map_err(|e| std::io::Error::new(ErrorKind::BrokenPipe, e))
        }
    }
    let mut send = WrapSender(send, Vec::new());
    let model = model.borrow();
    for idx in 0..25 {
        if let Some(uiday) = &model.days[idx] {
            if uiday.active {
                uiday.handler.run(&uiday.input, &mut send)
            }
        }
    }
}

fn build_big_run_button(model: Rc<RefCell<UIModel>>) -> Button {
    let button = Button::builder()
        .label("Run selected")
        .build();

    button.connect_clicked(
        move |b| {
            let (send, _) = channel::<String>();
            perform_run(model.clone(), send)
        }
    );

    button
}

fn build_ui(app: &Application, model: Rc<RefCell<UIModel>>) {
    let layout = LayoutBox::builder()
        .orientation(Orientation::Vertical)
        .spacing(3)
        .build();

    layout.append(&build_day_selector_grid(model.clone()));
    layout.append(&build_big_run_button(model.clone()));
    layout.append(&build_input_stack_pages(model));

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Advent of Code, 2024 Edition")
        .child(&layout)
        .build();

    window.present()
}

impl GtkUI {
    pub fn new() -> GtkUI {
        Self {}
    }
}

struct UIDay {
    active: bool,
    input: String,
    handler: Box<dyn Day>,
}

struct UIModel {
    days: [Option<UIDay>; 25],
}

impl UIModel {
    fn new(activations: &[u8], advent_of_code: AdventOfCode) -> Self {
        let mut days = [const { None }; 25];
        let iter = advent_of_code.days.into_iter()
            .zip(advent_of_code.inputs.into_iter())
            .enumerate();
        for (n, (solve, input)) in iter {
            if let Some(handler) = solve {
                let active = activations.iter().find(|it| **it == n as u8 + 1).is_some();
                days[n] = Some(UIDay {
                    handler,
                    input,
                    active,
                })
            }
        }

        UIModel { days }
    }
}

impl UI for GtkUI {
    fn run(&self, preselected_days: Vec<u8>, aoc: AdventOfCode) -> ExitCode {
        let app = Application::builder()
            .application_id("codecentric.aoc.AoC2024")
            .build();
        let model = Rc::new(RefCell::new(UIModel::new(&preselected_days, aoc)));
        app.connect_activate(clone!(
            #[weak] model,
            move |app| {
                build_ui(app, model)
            })
        );

        if app.run_with_args::<&str>(&[]).value() == 0 {
            ExitCode::SUCCESS
        } else {
            ExitCode::FAILURE
        }
    }
}