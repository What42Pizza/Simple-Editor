use crate::{data::{program_data::*, settings::*, errors::*, errors::Result::*}, fns};

use std::{thread::JoinHandle, sync::mpsc::Sender, result::Result as stdResult};



pub fn finish (program_data: &ProgramData, task_thread: JoinHandle<()>, tasks_tx: Sender<ProgramTask>) -> Result<()> {

    // set continue details

    // wait for threads
    drop(tasks_tx);
    let task_thread_result = task_thread.join();
    if let stdResult::Err(error) = task_thread_result {
        println!("Warning: tasks thread returned an error: {:?}", error);
    }

    Ok(())

}