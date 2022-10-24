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
        mpsc::{self, *}
    }
};
use sdl2::{EventPump,
    render::{TextureCreator, Canvas},
    video::{WindowContext, Window}
};

use crate::{data::{program_data::*, errors::*, errors::Result::*}, task_fns::tasks as tasks};



fn main() {

    // sdl
    let (sdl_context, canvas) = init::init_sdl2();
    let event_pump = sdl_context.event_pump().expect("Could not retrieve event pump");
    let texture_creator = canvas.texture_creator();

    let mut program_data = ProgramData::new();

    let success = run_program(&mut program_data, canvas, &texture_creator, event_pump);
    if let Err(error) = success {
        println!("\nError while running program: {}\n", error);
        println!("\n\n\nProgram data: {:#?}\n", program_data);
    }

}



fn run_program<'a> (program_data: &mut ProgramData<'a>, mut canvas: Canvas<Window>, texture_creator: &'a TextureCreator<WindowContext>, mut event_pump: EventPump) -> Result<()> {

    init::init_program_data(program_data, &texture_creator)?;
    
    let mut program_data_mutex = Arc::new(Mutex::new(program_data));
    let (tx, rx): (Sender<TaskUpdateInfo>, Receiver<TaskUpdateInfo>) = mpsc::channel();
    let task_thread = thread::spawn(move || tasks::run_tasks(tx));

    let mut last_update_instant = Instant::now();
    loop {

        let dt = last_update_instant.elapsed();
        last_update_instant = Instant::now();

        let exit = update::update(&mut program_data_mutex, &mut event_pump, &dt)?;
        render::render(&mut canvas, &program_data_mutex)?;

        if exit {break;}

    }

    let task_thread_result = task_thread.join();
    if let stdResult::Err(error) = task_thread_result {
        println!("Warning: tasks thread returned an error: {:?}", error);
    }

    Ok(())

}
