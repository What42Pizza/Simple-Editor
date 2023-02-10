use crate::prelude::*;



pub fn run_tasks (program_data: &ProgramData) {
    'outer: loop {

        // wait for tasks
        while program_data.tasks.read().is_empty() {
            thread::sleep(Duration::from_millis(10));
            if *program_data.exit.read() {break 'outer;}
        }

        // run tasks
        'inner: loop {
            let current_task = program_data.tasks.write().remove(0);
            match process_task(current_task, program_data) {
                Ok(_) => {}
                Err(error) => {
                    program_data.errors.write().push(error);
                }
            }
            if *program_data.exit.read() {break 'outer;}
            if program_data.tasks.read().is_empty() {break 'inner;}
        }

    }
}



pub fn process_task (current_task: ProgramTask, program_data: &ProgramData) -> Result<(), ProgramError> {

    match current_task {
        ProgramTask::LoadFile{file_path, switch_to_this} => load_file(&file_path, program_data)?,
        ProgramTask::SaveFile{file_num, file_path} => save_file(&file_path, program_data)?,
    }

    Ok(())
}





pub fn load_file (file_path: &str, program_data: &ProgramData) -> Result<(), ProgramError> {
    println!("Loading files {file_path}");

    let contents = match fs::read_to_string(file_path) {
        Ok(v) => v,
        Err(error) => return err(RawProgramError::CouldNotLoadFile {
            file_path: file_path.to_string(),
            source: error,
        }),
    };
    let contents = fns::split_lines(&contents);
    let new_file = File::new(file_path.to_string(), contents);
    program_data.files.write().push(new_file);

    let mut curent_file = program_data.current_file_num.write();
    if curent_file.is_none() {
        *curent_file = Some(0);
    }
    drop(curent_file);

    println!("loaded file {file_path}");
    Ok(())
}



pub fn save_file (file_path: &str, program_data: &ProgramData) -> Result<(), ProgramError> {
    println!("wip: save file {file_path}");
    Ok(())
}
