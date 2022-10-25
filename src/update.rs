use crate::{data::{program_data::*, settings::*, errors::*, errors::Result::*}, fns};

use std::{time::Duration};
use sdl2::{EventPump, event::Event, keyboard::Keycode};



pub fn update (program_data: &mut ProgramData, event_pump: &mut EventPump, dt: &Duration) -> Result<bool> {
    
    process_events(program_data, event_pump)?;

    temp_print_errors(program_data);

    Ok(*program_data.exit.lock().unwrap())

}



pub fn temp_print_errors (program_data: &mut ProgramData) {
    for error in program_data.errors.lock().unwrap().iter() {
        println!("Error: {}", error);
    }
    *program_data.errors.lock().unwrap() = vec!();
}



pub fn process_events (program_data: &mut ProgramData, event_pump: &mut EventPump) -> Result<()> {
    
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit {..} |
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                *program_data.exit.lock().unwrap() = true;
                return Ok(());
            },
            _ => {}
        }
    }

    Ok(())

}
