use crate::prelude::*;



pub fn run_tasks (program_data: ProgramData) {
    'outer: loop {

        // wait for tasks
        *program_data.tasks_ongoing.lock().unwrap() = false;
        while program_data.tasks.lock().unwrap().is_empty() || *program_data.tasks_paused.lock().unwrap() {
            thread::sleep(Duration::from_millis(10));
            if *program_data.exit.lock().unwrap() {break 'outer;}
        }
        *program_data.tasks_ongoing.lock().unwrap() = true;

        // run tasks
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
            if break_at_end || *program_data.tasks_paused.lock().unwrap() {break 'inner;}
        }

    }
}



pub fn process_task (current_task: ProgramTask, program_data: &ProgramData) -> Result<()> {

    match current_task {
        ProgramTask::LoadFile(file_path) => load_file(&file_path, program_data)?,
        ProgramTask::SaveFile(file_path) => save_file(&file_path, program_data)?,
        ProgramTask::CloseFile(file_path) => close_file(&file_path, program_data)?,
    }

    Ok(())
}





pub fn load_file (file_path: &str, program_data: &ProgramData) -> Result<()> {

    let contents = fs::read_to_string(file_path)
        .err_details_lazy(|| "Failed to read file \"".to_string() + file_path + "\"")?;
    let contents = fns::split_lines(&contents);
    program_data.files.lock().unwrap().push(File::new(file_path.to_string(), contents));

    let mut curent_file = program_data.current_file_num.lock().unwrap();
    if curent_file.is_none() {
        *curent_file = Some(0);
    }
    drop(curent_file);

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
