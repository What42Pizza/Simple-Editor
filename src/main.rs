// Started 10/21/22
// Last updated 01/13/23



// SDL2 docs: https://rust-sdl2.github.io/rust-sdl2/sdl2/
// hjson docs: https://docs.rs/serde-hjson/0.9.1/serde_hjson/



// default rust
#![allow(unused)]
#![warn(unused_must_use)]

// clippy
#![allow(clippy::too_many_arguments)]

// nightly features
#![feature(try_trait_v2)]
#![feature(type_alias_impl_trait)]



mod prelude;
mod update_mod;
mod render;
mod init;
mod unwind;
mod background_tasks_mod;
mod data_mod;
mod fns;



#[macro_use]
extern crate derive_is_enum_variant;



use crate::prelude::*;
use sdl2::{EventPump, event::Event, pixels::Color};





fn main() {

    let mut program_data = ProgramData::new();

    if let Err(error) = run_program(&mut program_data) {
        println!("\nError while running program: {}\n", error);
        println!("\n\n\nProgram data: {:#?}\n", program_data);
    }

}



fn run_program (program_data: &mut ProgramData) -> Result<()> {

    // init settings
    let settings = load_settings();
    *program_data.settings.borrow_mut() = Some(settings);

    // sdl
    let settings_ref = program_data.settings.borrow();
    let settings = settings_ref.as_ref().unwrap();
    let (sdl_context, ttf_context, mut canvas) = init::init_sdl2(&settings);
    let mut event_pump = sdl_context.event_pump().expect("Could not retrieve event pump");
    let texture_creator = canvas.texture_creator();
    drop(settings_ref);

    // main init
    let (font, mut tetuxres) = init::init_program_data(program_data, &texture_creator, &ttf_context)?;
    let thread_program_data = program_data.clone();
    let task_thread = thread::spawn(move || background_tasks::run_tasks(thread_program_data));

    // main loop
    let mut frame_count = 0;
    let mut last_frame_count_print = Instant::now();
    while !*program_data.exit.borrow() {

        update::update(program_data, &mut event_pump);
        render::render(&mut canvas, program_data, &mut tetuxres, &texture_creator, &font)?;

        frame_count += 1;
        if last_frame_count_print.elapsed().as_secs_f64() > 1. {
            println!("framerate: {frame_count}");
            frame_count = 0;
            last_frame_count_print = Instant::now();
        }

    }

    // finish and close
    unwind::unwind(program_data, task_thread)?;

    Ok(())

}
