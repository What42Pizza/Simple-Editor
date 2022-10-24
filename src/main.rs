// Started 10/21/22
// Last updated 10/23/22



// hjson docs: https://docs.rs/serde-hjson/0.9.1/serde_hjson/



// default rust
#![allow(unused)]
#![warn(unused_must_use)]

// nightly features
#![feature(try_trait_v2)]



mod update;
mod render;
mod init;
mod task_fns;
mod data;
mod fns;



#[macro_use]
extern crate derive_is_enum_variant;



use std::{time::Instant, thread, result::Result as stdResult,
    sync::{Arc, Mutex,
        mpsc::{self}
    }
};
use sdl2::{EventPump,
    render::{TextureCreator, Canvas},
    video::{WindowContext, Window}
};

use crate::{data::{program_data::*, errors::*, errors::Result::*}, task_fns::tasks as tasks};



fn main() {

    let mut program_data = Arc::new(Mutex::new(ProgramData::new()));

    if let Err(error) = run_program(&mut program_data) {
        println!("\nError while running program: {}\n", error);
        println!("\n\n\nProgram data: {:#?}\n", program_data);
    }

}



fn run_program (program_data: &mut Arc<Mutex<ProgramData>>) -> Result<()> {

    // sdl
    let (sdl_context, mut canvas) = init::init_sdl2();
    let mut event_pump = sdl_context.event_pump().expect("Could not retrieve event pump");
    let texture_creator = canvas.texture_creator();

    // main init
    let tetuxres = init::init_program_data(&mut program_data.lock().unwrap(), &texture_creator)?;

    // threading
    let thread_program_data_mutex = program_data.clone();
    let task_thread = thread::spawn(move || tasks::run_tasks(thread_program_data_mutex));

    // main loop
    let mut last_update_instant = Instant::now();
    loop {

        let dt = last_update_instant.elapsed();
        last_update_instant = Instant::now();

        let exit = update::update(program_data, &mut event_pump, &dt)?;
        if exit {return Ok(());}
        render::render(&mut canvas, &program_data, &tetuxres)?;

    }

    // wait for threads
    let task_thread_result = task_thread.join();
    if let stdResult::Err(error) = task_thread_result {
        println!("Warning: tasks thread returned an error: {:?}", error);
    }

    Ok(())

}
