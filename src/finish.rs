use crate::prelude::*;



pub fn finish (program_data: &ProgramData, task_thread: JoinHandle<()>) -> Result<()> {

    // set continue details

    // save settings

    // wait for threads
    let task_thread_result = task_thread.join();
    if let stdResult::Err(error) = task_thread_result {
        println!("Warning: tasks thread returned an error: {:?}", error);
    }

    Ok(())

}