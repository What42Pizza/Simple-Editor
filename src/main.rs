// Started 10/21/22
// Last updated 10/26/22



// SDL2 docs: https://rust-sdl2.github.io/rust-sdl2/sdl2/
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

use std::{thread,
    sync::mpsc::{Sender, self, Receiver}
};
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
    let (sdl_context, mut canvas) = init::init_sdl2();
    let mut event_pump = sdl_context.event_pump().expect("Could not retrieve event pump");
    let texture_creator = canvas.texture_creator();

    // main init
    let (tasks_tx, tasks_rx): (Sender<ProgramTask>, Receiver<ProgramTask>) = mpsc::channel();
    let tetuxres = init::init_program_data(program_data, &texture_creator, &tasks_tx)?;
    let thread_program_data_mutex = program_data.clone();
    let task_thread = thread::spawn(move || tasks::run_tasks(thread_program_data_mutex, tasks_rx));

    // main loop
    while !*program_data.exit.lock().unwrap() {

        update(program_data, &mut event_pump, &tasks_tx);

        render::render(&mut canvas, program_data, &tetuxres)?;

    }

    finish::finish(program_data, task_thread, tasks_tx)?;

    Ok(())

}



fn update (program_data: &mut ProgramData, event_pump: &mut EventPump, tasks_tx: &Sender<ProgramTask>) {

    *program_data.frame_count.lock().unwrap() += 1;

    for event in event_pump.poll_iter() {
        let _ = tasks_tx.send(ProgramTask::HandleEvent(event));
    }

}
