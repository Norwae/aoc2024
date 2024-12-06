use std::io::{ErrorKind, Write};
use std::mem::swap;
use std::process::ExitCode;
use std::rc::Rc;
use gtk4::{Application, ApplicationWindow, Grid, Box as LayoutBox, Orientation, CheckButton, Button, Widget, StackSidebar, Stack, Separator, TextView, TextBuffer, ScrolledWindow};
use gtk4::glib;
use gtk4::glib::*;
use gtk4::prelude::*;

use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
use std::time::Duration;
use crate::Configuration;
use crate::day::{Day, handlers};

use crate::worker::run_on_worker;

static HANDLERS: [Option<Day<WrapSender>>; 25] = handlers();

fn build_day_selector_widget(idx: u8, preselect: bool) -> CheckButton {
    let handler = &HANDLERS[idx as usize];
    CheckButton::builder()
        .label(format!("Day {}", idx + 1))
        .active(preselect && handler.is_some())
        .sensitive(handler.is_some())
        .build()
}

fn build_input_editor(initial: &str) -> ScrolledWindow {
    let buffer = TextBuffer::builder()
        .enable_undo(true)
        .text(initial)
        .build();

    let text = TextView::builder()
        .buffer(&buffer)
        .monospace(true)
        .build();
    ScrolledWindow::builder()
        .height_request(500)
        .width_request(500)
        .child(&text)
        .build()
}

fn build_input_stack_pages(input_source: &Configuration) -> (LayoutBox, StackSidebar) {
    let layout = LayoutBox::new(Orientation::Horizontal, 2);
    let stack = Stack::new();
    let separator = Separator::new(Orientation::Vertical);
    let selector = StackSidebar::builder()
        .stack(&stack)
        .width_request(100)
        .height_request(400)
        .build();

    for d in HANDLERS.iter().enumerate() {
        if let (idx, Some(_)) = d {
            let input_editor = build_input_editor(&String::from_utf8(input_source.load_input((idx + 1) as u8)).unwrap());
            let name = format!("day_{}", idx);
            let label = format!("Day {}", idx + 1);
            stack.add_titled(&input_editor, Some(&name), &label);
        }
    }

    layout.append(&selector);
    layout.append(&separator);
    layout.append(&stack);

    (layout, selector)
}

fn build_day_selector_grid(preselect: &[u8]) -> Grid {
    let grid = Grid::builder()
        .column_homogeneous(true)
        .column_spacing(2)
        .build();
    for idx in 0u8..=24 {
        let selector_widget = build_day_selector_widget(idx, preselect.contains(&(idx + 1)));
        grid.attach(&selector_widget, (idx as i32) % 5, (idx as i32) / 5, 1, 1)
    }

    grid
}

fn active_from_day_selector_grid(grid: &Grid) -> Vec<usize> {
    let mut result = Vec::new();
    for row in 0..5 {
        for col in 0..5 {
            let button = grid.child_at(col, row)
                .expect("Widget at location")
                .downcast::<CheckButton>()
                .expect("Widget is checkbutton");
            if button.is_active() {
                result.push((5 * row + col) as usize)
            }
        }
    }
    result
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

fn input_from_input_sidebar(sidebar: &StackSidebar, day: usize) -> String {
    let buffer = sidebar.stack().expect("Stack installed")
        .child_by_name(&format!("day_{}", day)).expect("Child present")
        .downcast::<ScrolledWindow>().expect("Scroller")
        .child().expect("Text present")
        .downcast::<TextView>().expect("Text")
        .buffer();
    buffer.text(&mut buffer.start_iter(), &mut buffer.end_iter(), false).to_string()
}

fn perform_run(text: TextBuffer, grid: Grid, sidebar: StackSidebar, verbose: CheckButton) {
    let (sender, receiver) = channel();
    let run_verbose = verbose.is_active();

    for day in active_from_day_selector_grid(&grid) {
        let input = input_from_input_sidebar(&sidebar, day);
        let Day { terse, verbose } = HANDLERS[day].as_ref()
            .expect("Active days are available");
        let mut wrapper = WrapSender(sender.clone(), Vec::new());
        run_on_worker(move || {
            if run_verbose {
                verbose(input.as_bytes(), &mut wrapper)
            } else {
                terse(input.as_bytes(), &mut wrapper)
            }
        });
    }

    install_ui_update_callback(receiver, text)
}

fn install_ui_update_callback(recv: Receiver<String>, text: TextBuffer) {
    timeout_add_local(Duration::from_millis(100),
                      move || {
                          let mut iter = text.end_iter();
                          for _ in 0..50 {
                              match recv.try_recv() {
                                  Ok(msg) => {
                                      text.insert(&mut iter, &msg);
                                  }
                                  Err(TryRecvError::Empty) => return ControlFlow::Continue,
                                  _ => return ControlFlow::Break
                              }
                          }

                          return ControlFlow::Continue;
                      });
}

fn build_big_run_button(text: TextBuffer, grid: Grid, sidebar: StackSidebar, verbose: CheckButton) -> Button {
    let button = Button::builder()
        .label("Run selected")
        .build();

    button.connect_clicked(clone!(
        #[weak] text,
        #[weak] grid,
        #[weak] sidebar,
        #[weak] verbose,
        move |_b| {
            perform_run(text, grid, sidebar, verbose)
        }
    ));

    button
}

fn build_verbose_control(initial: bool) -> CheckButton {
    CheckButton::builder()
        .active(initial)
        .label("Verbose")
        .build()
}

fn build_ui(app: &Application, config: &Configuration) {

    let layout = Grid::builder()
        .column_spacing(4)
        .build();
    let day_selector_grid = build_day_selector_grid(config.active_days());
    layout.attach(&day_selector_grid, 0, 0, 2, 1);
    let verbose = build_verbose_control(config.verbose);
    layout.attach(&verbose, 0, 1, 1, 1);
    let (page_box, selector) = build_input_stack_pages(&config);
    layout.attach(&page_box, 0, 2, 2, 1);
    let (text, widget) = build_output_view();
    layout.attach(&widget, 0, 3, 2, 1);
    let button = build_big_run_button(text, day_selector_grid, selector, verbose);
    layout.attach(&button, 1, 1, 1, 1);

    ApplicationWindow::builder()
        .application(app)
        .title("Advent of Code, 2024 Edition")
        .child(&layout)
        .build()
        .present()
}

fn build_output_view() -> (TextBuffer, Widget) {
    let buffer = TextBuffer::new(None);
    let widget = TextView::builder()
        .monospace(true)
        .height_request(300)
        .editable(false)
        .buffer(&buffer)
        .build();
    let widget = ScrolledWindow::builder()
        .width_request(500)
        .height_request(200)
        .child(&widget)
        .build();

    (buffer, widget.upcast())
}

pub fn gtk_run(config: Configuration) -> ExitCode {
    let config = Rc::new(config);
    let app = Application::builder()
        .application_id("codecentric.aoc.AoC2024")
        .build();
    app.connect_activate(clone!(#[strong] config, move |app| {
        build_ui(app, &config)
    }));

    if app.run_with_args::<&str>(&[]).value() == 0 {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
