use crate::{data::{program_data::*, errors::*, errors::Result::*}, fns};

use std::{time::Duration,
    sync::{Arc, Mutex}
};
use sdl2::{EventPump, event::Event, keyboard::Keycode};



pub fn update (program_data: &mut Arc<Mutex<&mut ProgramData>>, event_pump: &mut EventPump, dt: &Duration) -> Result<bool> {
    let mut program_data = program_data.lock().unwrap();
    
    process_events(&mut program_data, event_pump)?;

    Ok(program_data.exit)

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
