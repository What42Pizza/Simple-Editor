use crate::{data::{program_data::*, settings::*, errors::*, errors::Result::*}, fns};

use sdl2::{event::Event, keyboard::Keycode};
use std::{fs, thread, time::{Duration, Instant}};



pub fn run_tasks (program_data: ProgramData) {
    'outer: loop {

        while program_data.tasks.lock().unwrap().is_empty() {
            thread::sleep(Duration::from_millis(10));
            if *program_data.exit.lock().unwrap() {break 'outer;}
        }

        'inner: loop {
            let mut tasks = program_data.tasks.lock().unwrap();
            let current_task = tasks.remove(0);
            let break_at_end = tasks.is_empty();
            drop(tasks);
            match process_task(current_task, &program_data) {
                Ok(_) => {}
                Err(error) => {
                    program_data.errors.lock().unwrap().push(error);
                }
            }
            if *program_data.exit.lock().unwrap() {break 'outer;}
            if break_at_end {break 'inner;}
        }

    }
}



pub fn process_task (current_task: ProgramTask, program_data: &ProgramData) -> Result<()> {

    match current_task {
        ProgramTask::DoFrameUpdates => do_frame_updates(program_data)?,
        ProgramTask::LoadFile(file_path) => load_file(&file_path, program_data)?,
        ProgramTask::SaveFile(file_path) => save_file(&file_path, program_data)?,
        ProgramTask::CloseFile(file_path) => close_file(&file_path, program_data)?,
        ProgramTask::HandleEvent(event) => handle_event(event, program_data)?,
    }

    Ok(())
}





pub fn do_frame_updates (program_data: &ProgramData) -> Result<()> {

    let current_time = Instant::now();
    let mut last_frame_updates_time = program_data.last_frame_updates_time.lock().unwrap();
    let dt = current_time.duration_since(*last_frame_updates_time);
    *last_frame_updates_time = current_time;

    let mut frame_update_fns = program_data.frame_update_fns.lock().unwrap();
    while !frame_update_fns.is_empty() {
        frame_update_fns.pop().unwrap()(program_data, &dt);
    }

    Ok(())
}





pub fn load_file (file_path: &str, program_data: &ProgramData) -> Result<()> {

    let contents = fs::read_to_string(file_path)
        .err_details_lazy(|| "Failed to read file \"".to_string() + file_path + "\"")?;
    let contents = fns::split_lines(&contents);
    program_data.files.lock().unwrap().push(File::new(file_path.to_string(), contents));

    if program_data.current_file.lock().unwrap().is_none() {
        *program_data.current_file.lock().unwrap() = Some(0);
    }

    println!("loaded file {}", file_path);
    Ok(())
}



pub fn save_file (file_path: &str, program_data: &ProgramData) -> Result<()> {
    println!("wip: save file {}", file_path);
    Ok(())
}



pub fn close_file (file_path: &str, program_data: &ProgramData) -> Result<()> {
    println!("wip: close file {}", file_path);
    Ok(())
}



pub fn handle_event (event: Event, program_data: &ProgramData) -> Result<()> {
    match event {
        Event::Quit {..} |
        Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
            *program_data.exit.lock().unwrap() = true;
            return Ok(());
        },
        _ => {}
    }
    Ok(())
}
