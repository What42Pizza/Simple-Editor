use crate::prelude::*;





pub fn update (program_data: &mut ProgramData, event_pump: &mut EventPump) {

    handle_events(program_data, event_pump);

    let mut errors = program_data.errors.borrow();
    for error in errors.iter() {
        println!("ERROR: {error}");
    }
    drop(errors);
    program_data.errors.borrow_mut().clear();

    *program_data.frame_count.borrow_mut() += 1;
}



pub fn handle_events (program_data: &mut ProgramData, event_pump: &mut EventPump) {

    // get list of events and re-order as needed
    let mut events = vec!();
    for event in event_pump.poll_iter() {
        insert_event(event, &mut events);
    }

    // handle events (in correct order)
    for event in events {
        events::handle_event(event, program_data);
    }

}



pub fn insert_event (event: Event, events: &mut Vec<Event>) {

    if events.is_empty() {
        events.push(event);
        return;
    }

    if let Event::TextInput {timestamp: text_input_timestamp, ..} = event {
        let last_event = &events[events.len() - 1];
        if let Event::KeyDown {timestamp: key_down_timestamp, ..} = last_event {
            if text_input_timestamp == *key_down_timestamp {
                let index = events.len() - 1;
                events.insert(index, event);
                return;
            }
        }
    }
    
    events.push(event);

}
