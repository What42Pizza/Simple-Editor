use crate::{data::{program_data::*, errors::*, errors::Result::*}, fns};

use std::{time::Duration};
use sdl2::{EventPump, event::Event, keyboard::Keycode};



pub fn update (program_data: &mut ProgramData, event_pump: &mut EventPump, dt: &Duration) -> Result<()> {
    
    process_events(program_data, event_pump)?;

    Ok(())

}



fn process_events (program_data: &mut ProgramData, event_pump: &mut EventPump) -> Result<()> {
    
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit {..} |
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                program_data.exit = true;
                return Ok(());
            },
            _ => {}
        }
    }

    Ok(())

}
