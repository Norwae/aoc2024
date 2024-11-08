use std::cell::RefCell;
use std::io::{ErrorKind, Write};
use std::mem::swap;
use std::process::ExitCode;
use std::rc::Rc;
use gtk4::{Application, ApplicationWindow, Grid, Box as LayoutBox, Orientation, CheckButton, Button, Label, Widget, StackSidebar, Stack, Separator, TextView, TextBuffer, ScrolledWindow, Align};
use gtk4::glib;
use gtk4::glib::*;
use gtk4::prelude::*;

use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
use std::thread;
use std::time::Duration;
use crate::day::handlers;

use crate::Inputs;
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
                .halign(Align::Start)
                .build()
                .upcast()
    }
}

fn build_input_editor(n: usize, day: &UIDay, model: Rc<RefCell<UIModel>>) -> Widget {
    let contents = day.input.to_string();
    let buffer = TextBuffer::builder()
        .enable_undo(true)
        .text(contents)
        .build();

    let text = TextView::builder()
        .buffer(&buffer)
        .monospace(true)
        .build();
    let scroller = ScrolledWindow::builder()
        .height_request(500)
        .width_request(500)
        .child(&text)
        .build();


    buffer.connect_changed(clone!(
        #[weak] model,
        move |txt| {
            let text = txt.text(&txt.start_iter(), &txt.end_iter(), false);
            model.borrow_mut().days[n].as_mut().unwrap().input = text.to_string();
        })
    );

    scroller.upcast()
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
struct WrapSender(Sender<String>, Vec<u8>);
impl Write for WrapSender {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.1.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        let mut captured = Vec::new();
        swap(&mut captured, &mut self.1);
        let string = String::from_utf8(captured).expect("UTF8");
        self.0.send(string).map_err(|e| std::io::Error::new(ErrorKind::BrokenPipe, e))
    }
}

fn perform_run(model: Rc<RefCell<UIModel>>, sender: Sender<String>)  {

    for idx in 0..25 {
        let model = model.borrow();
        if let Some(uiday) = &model.days[idx] {
            if uiday.active {
                let mut wrapper = WrapSender(sender.clone(), Vec::new());
                let day = handlers::<WrapSender>(!model.verbose)[uiday.index]().expect("Handler available");
                let input = uiday.input.clone();
                thread::spawn(move ||{
                    (day.handler)(&input, &mut wrapper)
                });
            }
        }
    }
}

fn build_big_run_button(model: Rc<RefCell<UIModel>>, sender: Sender<String>) -> Button {
    let button = Button::builder()
        .label("Run selected")
        .build();

    button.connect_clicked(
        move |_b| {
            perform_run(model.clone(), sender.clone());
        }
    );

    button
}

fn build_verbose_control(model: Rc<RefCell<UIModel>>) -> Widget {
    let button = CheckButton::builder()
        .active(model.borrow().verbose)
        .label("Verbose")
        .build();

    button.connect_toggled(clone!(
        #[weak] model,
        move |b|{
            model.borrow_mut().verbose = b.is_active();
        }
    ));
    button.upcast()
}

fn build_ui(app: &Application, model: Rc<RefCell<UIModel>>) {
    let (send, receive) = channel::<String>();
    let layout = Grid::builder()
        .column_spacing(4)
        .build();
    layout.attach(&build_day_selector_grid(model.clone()), 0, 0, 2, 1);
    layout.attach(&build_verbose_control(model.clone()), 0, 1, 1, 1);
    layout.attach(&build_big_run_button(model.clone(), send), 1, 1, 1, 1);
    layout.attach(&build_input_stack_pages(model), 0, 2, 2, 1);
    layout.attach(&build_output_view(receive), 0, 3, 2, 1);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Advent of Code, 2024 Edition")
        .child(&layout)
        .build();

    window.present()
}

fn build_output_view(input_channel: Receiver<String>) -> Widget {
    let buffer = Box::leak(Box::new(TextBuffer::new(None)));
    let buffer_ptr: *mut TextBuffer = buffer;
    let buffer_ptr = buffer_ptr as usize;
    let widget = TextView::builder()
        .monospace(true)
        .height_request(300)
        .editable(false)
        .buffer(buffer)
        .build();
    let widget= ScrolledWindow::builder()
        .width_request(500)
        .height_request(200)
        .child(&widget)
        .build();

    timeout_add(Duration::from_millis(100),  move||{
        let buffer = unsafe {
            &*(buffer_ptr as *mut TextBuffer)
        };
        let mut result = input_channel.try_recv();
        let mut count = 0;
        while let Ok(text) = &result {
            let mut end = buffer.end_iter();
            buffer.insert(&mut end, text);
            count += 1;
            if count >= 100 {
                // force loop to terminate so UI can unblock
                break;
            } else {
                result = input_channel.try_recv();
            }
        }

        if result == Err(TryRecvError::Disconnected) {
            ControlFlow::Break
        } else {
            ControlFlow::Continue
        }
    });

    widget.upcast()
}
struct UIDay {
    index: usize,
    active: bool,
    input: String,
}

struct UIModel {
    verbose: bool,
    days: [Option<UIDay>; 25],
}

impl UIModel {
    fn new(activations: &[u8], advent_of_code: Inputs, verbose: bool) -> Self {
        let mut days = [const { None }; 25];

        let iter = handlers::<WrapSender>(!verbose).into_iter()
            .zip(advent_of_code.inputs.into_iter())
            .enumerate();
        for (n, (solver, input)) in iter {
            if let Some(_) = solver() {
                let active = activations.iter().find(|it| **it == n as u8 + 1).is_some();
                days[n] = Some(UIDay {
                    index: n,
                    input,
                    active,
                })
            }
        }

        UIModel { verbose, days }
    }
}

impl UI for GtkUI {
    fn run(&self, preselected_days: &[u8], aoc: Inputs, verbose: bool) -> ExitCode {
        let app = Application::builder()
            .application_id("codecentric.aoc.AoC2024")
            .build();
        let model = Rc::new(RefCell::new(UIModel::new(&preselected_days, aoc, verbose)));
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