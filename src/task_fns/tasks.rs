use crate::{data::{program_data::*, settings::*, errors::*, errors::Result::*}, fns};

use std::{thread, time::Duration};



pub fn run_tasks (program_data: ProgramData) {
    while !*program_data.exit.lock().unwrap() {

        'tasks_loop: while !*program_data.exit.lock().unwrap() {
            let mut tasks = program_data.tasks.lock().unwrap();
            if tasks.is_empty() {break 'tasks_loop;}
            let current_task = tasks.remove(0);
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
        ProgramTask::LoadFile(file_name) => load_file(&*file_name, program_data)?,
    }

    Ok(())
}





pub fn load_file (file_name: &str, program_data: &ProgramData) -> Result<()> {

    Ok(())
}
