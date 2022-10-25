use crate::{data::{program_data::*, errors::*, errors::Result::*}, fns};



#[derive(Debug)]
pub struct ProgramSettings {
    pub continue_details: ContinueDetails,
}

#[derive(Debug)]
pub struct ContinueDetails {
    pub last_open_files: Vec<String>,
}