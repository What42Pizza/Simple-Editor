use crate::prelude::*;





pub fn update (program_data: &ProgramData, event_pump: &mut EventPump) -> Result<(), ProgramError> {

    handle_events(program_data, event_pump)?;

    let errors = program_data.errors.read();
    for error in errors.iter() {
        println!("ERROR: {error:?}");
    }
    drop(errors);
    program_data.errors.write().clear();

    *program_data.frame_count.write() += 1;

    Ok(())
}



pub fn handle_events (program_data: &ProgramData, event_pump: &mut EventPump) -> Result<(), ProgramError> {

    // get list of events and re-order as needed
    let mut events = vec!();
    let mut text_input_events = vec!();
    for event in event_pump.poll_iter() {
        match event {
            Event::TextInput {..} => text_input_events.push(event),
            _ => events.push(event),
        }
    }

    // handle events (in correct order)
    for event in chain!(text_input_events, events) {
        events::handle_event(event, program_data)?;
    }

    Ok(())
}
