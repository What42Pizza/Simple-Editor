use crate::{data::{program_data::*, settings::*, errors::*, errors::Result::*}, fns};

use std::{thread, time::Duration, fs};



pub fn run_tasks (program_data: ProgramData) {
    while !*program_data.exit.lock().unwrap() {

        'tasks_loop: while !*program_data.exit.lock().unwrap() {
            let mut tasks = program_data.tasks.lock().unwrap();
            if tasks.is_empty() {break 'tasks_loop;}
            let current_task = tasks.remove(0);
            drop(tasks); // idk if this does what I hope it does
            let success = process_task(current_task, &program_data);
            if let Err(error) = success {
                program_data.errors.lock().unwrap().push(error);
            }
        }

        'wait_loop: while !*program_data.exit.lock().unwrap() {
            thread::sleep(Duration::from_millis(10));
            let mut tasks = program_data.tasks.lock().unwrap();
            if !tasks.is_empty() {break 'wait_loop;}
        }

    }
}



pub fn process_task (current_task: ProgramTask, program_data: &ProgramData) -> Result<()> {

    match current_task {
        ProgramTask::LoadFile(file_path) => load_file(&*file_path, program_data)?,
        ProgramTask::SaveFile(file_path) => save_file(&*file_path, program_data)?,
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
    println!("loaded file {}", file_path);
    Ok(())
}



pub fn save_file (file_path: &str, program_data: &ProgramData) -> Result<()> {
    println!("wip: save file {}", file_path);
    Ok(())
}
