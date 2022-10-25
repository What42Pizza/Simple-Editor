use crate::{data::{program_data::*, settings::*, errors::*, errors::Result::*}, fns};

use sdl2::{event::Event, keyboard::Keycode};
use std::{fs, sync::mpsc::Receiver};



pub fn run_tasks (program_data: ProgramData, tasks_rx: Receiver<ProgramTask>) {
    for current_task in tasks_rx {
        let success = process_task(current_task, &program_data);
        if let Err(error) = success {
            program_data.errors.lock().unwrap().push(error);
        }
    }
}



pub fn process_task (current_task: ProgramTask, program_data: &ProgramData) -> Result<()> {

    match current_task {
        ProgramTask::LoadFile(file_path) => load_file(&*file_path, program_data)?,
        ProgramTask::SaveFile(file_path) => save_file(&*file_path, program_data)?,
        ProgramTask::CloseFile(file_path) => close_file(&*file_path, program_data)?,
        ProgramTask::HandleEvent(event) => handle_event(event, program_data)?
    }

    Ok(())
}





pub fn load_file (file_path: &str, program_data: &ProgramData) -> Result<()> {

    let contents = fs::read_to_string(&file_path)
        .err_details_lazy(|| "Failed to read file \"".to_string() + file_path + "\"")?
        .split('\n')
        .map(|s| s.to_string())
        .collect();
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
