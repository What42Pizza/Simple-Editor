use crate::prelude::*;





pub fn update (program_data: &mut ProgramData, event_pump: &mut EventPump) {

    handle_events(program_data, event_pump);

    *program_data.frame_count.lock().unwrap() += 1;
}



pub fn handle_events (program_data: &mut ProgramData, event_pump: &mut EventPump) {

    let mut events = vec!();
    for event in event_pump.poll_iter() {
        insert_event(event, &mut events);
    }

    let mut tasks = program_data.tasks.lock().unwrap();
    for event in events {
        events::handle_event(event, program_data);
    }
    drop(tasks);

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
