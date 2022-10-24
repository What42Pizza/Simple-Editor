use crate::{data::{program_data::*, errors::*, errors::Result::*}, fns};

use std::{thread, time::Duration};



pub fn run_tasks (program_data: ProgramData) {
    while !*program_data.exit.lock().unwrap() {

        let mut tasks = program_data.tasks.lock().unwrap();
        if tasks.is_empty() {
            thread::sleep(Duration::from_millis(100));
            continue;
        }
        let current_task = tasks.remove(0);

        let success = process_task (current_task, &program_data);
        if let Err(error) = success {
            program_data.errors.lock().unwrap().push(error);
        }

    }
}



pub fn process_task (current_task: ProgramTask, program_data: &ProgramData) -> Result<()> {

    Ok(())
}
