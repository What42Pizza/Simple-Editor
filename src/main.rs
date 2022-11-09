// Started 10/21/22
// Last updated 11/08/22



// SDL2 docs: https://rust-sdl2.github.io/rust-sdl2/sdl2/
// hjson docs: https://docs.rs/serde-hjson/0.9.1/serde_hjson/



// default rust
#![allow(unused)]
#![warn(unused_must_use)]

// clippy
#![allow(clippy::too_many_arguments)]

// nightly features
#![feature(try_trait_v2)]



mod prelude;
mod render;
mod init;
mod finish;
mod tasks_mod;
mod data_mod;
mod fns;



#[macro_use]
extern crate derive_is_enum_variant;



use crate::prelude::*;
use sdl2::{EventPump, event::Event};





fn main() {

    let mut program_data = ProgramData::new();

    if let Err(error) = run_program(&mut program_data) {
        println!("\nError while running program: {}\n", error);
        println!("\n\n\nProgram data: {:#?}\n", program_data);
    }

}



fn run_program (program_data: &mut ProgramData) -> Result<()> {

    // sdl
    let (sdl_context, ttf_context, mut canvas) = init::init_sdl2();
    let mut event_pump = sdl_context.event_pump().expect("Could not retrieve event pump");
    let texture_creator = canvas.texture_creator();

    // main init
    let (font, mut tetuxres) = init::init_program_data(program_data, &texture_creator, &ttf_context)?;
    let thread_program_data_mutex = program_data.clone();
    let task_thread = thread::spawn(move || tasks::run_tasks(thread_program_data_mutex));

    // main loop
    while !*program_data.exit.lock().unwrap() {
        update(program_data, &mut event_pump);
        render::render(&mut canvas, program_data, &mut tetuxres, &texture_creator, &font)?;
    }

    finish::finish(program_data, task_thread)?;

    Ok(())

}



fn update (program_data: &mut ProgramData, event_pump: &mut EventPump) {

    *program_data.frame_count.lock().unwrap() += 1;

    let mut tasks = program_data.tasks.lock().unwrap();
    for event in event_pump.poll_iter() {
        add_event_to_tasks(event, &mut tasks);
    }
    drop(tasks);

    add_frame_update_task(program_data);

}



fn add_event_to_tasks (event: Event, tasks: &mut MutexGuard<Vec<ProgramTask>>) {

    // swap TextInput events with KeyDown events
    if let Event::TextInput {timestamp: text_input_timestamp, ..} = event {
        let last_task = &tasks[tasks.len() - 1];
        if let ProgramTask::HandleEvent (last_event) = last_task {
            if let Event::KeyDown {timestamp: key_down_timestamp, ..} = last_event {
                if text_input_timestamp == *key_down_timestamp {
                    let index = tasks.len() - 1;
                    tasks.insert(index, ProgramTask::HandleEvent(event));
                    return;
                }
            }
        }
    }

    tasks.push(ProgramTask::HandleEvent(event));

}



fn add_frame_update_task (program_data: &mut ProgramData) {
    let mut has_frame_update_task = program_data.has_frame_update_task.lock().unwrap();
    if *has_frame_update_task {return;}
    *has_frame_update_task = true;
    program_data.tasks.lock().unwrap().push(ProgramTask::DoFrameUpdates);
}
