mod memory;
mod process;

use crate::memory::{AlgorithmType, Memory};
use crate::process::Process;
use fltk::{
    app, button,
    enums::{Color, FrameType},
    frame::Frame,
    group,
    prelude::*,
    window::Window,
};
use fltk_theme::{ThemeType, WidgetTheme};

fn main() {
    let a = app::App::default();
    let widget_scheme = WidgetTheme::new(ThemeType::Blue);
    widget_scheme.apply();

    let mut time: i32 = 0;
    let mut mem1 = Memory::new("First Fit");
    let mut mem2 = Memory::new("Best Fit");
    let mut mem3 = Memory::new("Worst Fit");

    let mut wind = Window::new(100, 100, 790, 740, "Memory Visualizer");

    let mut first_fit_display = init_memory_display(&mut wind, 10, mem1.name);
    let mut best_fit_display = init_memory_display(&mut wind, 270, mem2.name);
    let mut worst_fit_display = init_memory_display(&mut wind, 530, mem3.name);

    mem1.buttons = first_fit_display.0;
    mem2.buttons = best_fit_display.0;
    mem3.buttons = worst_fit_display.0;

    let mut info = init_info_panel(&mut wind, &mut time);

    wind.end();
    wind.show();
    wind.redraw();

    info.0.set_callback(move |_| {
        step(&mut mem1, AlgorithmType::First, &mut first_fit_display.1);
        step(&mut mem2, AlgorithmType::Best, &mut best_fit_display.1);
        step(&mut mem3, AlgorithmType::Worst, &mut worst_fit_display.1);
        time += 1;
        increment_time(&mut info.1, time);
    });

    a.run().unwrap();
}

fn increment_time(frame: &mut Frame, time: i32) {
    frame.set_label(format!("Time Elapsed: {} seconds", time).as_str());
    app::unlock();
}

fn step(memory_block: &mut Memory, algo_type: AlgorithmType, frame: &mut Frame) {
    memory_block.decrement_runtimes();

    let peek = memory_block.peek_process(algo_type);
    if peek.0 {
        let mut process = memory_block.get_process().unwrap();
        let bc = process.block_color;
        process.mem_position = peek.1;
        memory_block.allocate_memory(peek.1, &process.mem_size, bc);
        memory_block.running_processes.push(process);
    }

    memory_block.deallocate_memory();
    update_display(memory_block, frame);
}

// ===============================
//  ______
// < GUIs >
//  ------
//         \   ^__^
//          \  (xx)\_______
//             (__)\       )\/\
//              U  ||----w |
//                 ||     ||
// ===============================

fn update_display(memory_block: &mut Memory, frame: &mut Frame) {
    let to_allocate = memory_block.get_allocated_chunks();
    let to_deallocate = memory_block.get_deallocated_chunks();
    let next_process = memory_block.waiting_processes.clone();
    let running_processes = memory_block.running_processes.clone();
    let next_process = next_process.last();
    update_memory_display(
        memory_block,
        to_allocate,
        true,
        &running_processes,
        next_process,
        frame,
    );
    update_memory_display(
        memory_block,
        to_deallocate,
        false,
        &running_processes,
        next_process,
        frame,
    );
}

fn init_info_panel(window: &mut Window, time_elapsed: &mut i32) -> (button::Button, Frame) {
    let mut info = Window::new(0, 0, 790, 50, "");

    let step_button = button::Button::new(50, 15, 170, 30, "Step");
    info.end();

    let mut time_frame = Frame::new(275, 20, 250, 20, "");
    time_frame.set_label(format!("Time Elapsed: {} seconds", time_elapsed).as_str());

    info.add(&time_frame);
    window.add(&info);

    (step_button, time_frame)
}

fn init_memory_display(
    window: &mut Window,
    x_pos: i32,
    name: &str,
) -> (Vec<button::Button>, Frame) {
    let memory = Window::new(x_pos, 60, 250, 600, "");
    let mut memory_chunks = Vec::new();
    let mut hpack = group::Pack::default().with_size(250, 600);
    hpack.set_type(group::PackType::Vertical);
    for _ in 0..50 {
        let mut but = button::Button::default();
        but.set_color(Color::FrameDefault);
        memory_chunks.push(but);
    }
    hpack.end();
    hpack.auto_layout();
    memory.end();

    let mut type_frame = Frame::new(x_pos, 665, 250, 20, "");
    type_frame.set_label(name);

    let mut next_process_frame = Frame::new(x_pos, 685, 250, 50, "");
    next_process_frame.set_label("Next Process: N/A");

    window.add(&memory);
    window.add(&type_frame);
    window.add(&next_process_frame);
    (memory_chunks, next_process_frame)
}

fn update_memory_display(
    mem: &mut Memory,
    indicies: Vec<usize>,
    should_allocate: bool,
    running_processes: &[Process],
    next_process: Option<&Process>,
    frame: &mut Frame,
) {
    for i in indicies {
        let c = mem.mut_but(i);

        if should_allocate {
            for process in running_processes {
                if i == process.mem_position as usize && process.pid > 0 {
                    c.set_label(
                        format!(
                            "PID {} : SIZE {} : RUNTIME {}",
                            process.pid, process.mem_size, process.runtime
                        )
                        .as_str(),
                    );
                }
            }
            c.set_frame(FrameType::FlatBox);
        } else {
            c.set_color(Color::FrameDefault);
            c.set_frame(FrameType::EngravedBox);
            c.set_label("");
        }
        c.redraw();
    }

    if let Some(p) = next_process {
        frame.set_label(format!("Next Process:\nPID: {}\nSIZE: {}", p.pid, p.mem_size).as_str());
    } else {
        frame.set_label("Next Process: N/A");
    }
}
