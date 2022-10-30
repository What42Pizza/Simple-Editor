// Started 10/21/22
// Last updated 10/29/22



// SDL2 docs: https://rust-sdl2.github.io/rust-sdl2/sdl2/
// SDL2 tff video: https://www.youtube.com/watch?v=vVJIYaX3Kjw&t=169s
// hjson docs: https://docs.rs/serde-hjson/0.9.1/serde_hjson/



// default rust
#![allow(unused)]
#![warn(unused_must_use)]

// nightly features
#![feature(try_trait_v2)]



mod render;
mod init;
mod finish;
mod task_fns;
mod data;
mod fns;



#[macro_use]
extern crate derive_is_enum_variant;



use crate::{data::{program_data::*, settings::*, errors::*, errors::Result::*}, task_fns::tasks as tasks};

use std::{thread};
use sdl2::{EventPump};





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
    let (font, tetuxres) = init::init_program_data(program_data, &texture_creator, &ttf_context)?;
    let thread_program_data_mutex = program_data.clone();
    let task_thread = thread::spawn(move || tasks::run_tasks(thread_program_data_mutex));

    // main loop
    while !*program_data.exit.lock().unwrap() {

        update(program_data, &mut event_pump);

        render::render(&mut canvas, program_data, &tetuxres, &texture_creator, &font)?;

    }

    finish::finish(program_data, task_thread)?;

    Ok(())

}



fn update (program_data: &mut ProgramData, event_pump: &mut EventPump) {

    *program_data.frame_count.lock().unwrap() += 1;

    let mut tasks = program_data.tasks.lock().unwrap();
    for event in event_pump.poll_iter() {
        tasks.push(ProgramTask::HandleEvent(event));
    }
    drop(tasks);

}
