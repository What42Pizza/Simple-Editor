use std::{fmt::{self, Debug}};
use sdl2::{render::Texture};
use serde_hjson::{Map, Value};



#[derive(Debug)]
pub struct ProgramData {

    pub frame_count: u32, // overflows after ~10,000 hours at 120 fps
    pub exit: bool,
    pub settings: Option<ProgramSettings>,

    pub tasks: Vec<ProgramTask>,

    pub open_files: Vec<File>,

}

impl ProgramData {
    pub fn new() -> Self {
        Self {

            frame_count: 0,
            exit: false,
            settings: None,

            tasks: vec!(),

            open_files: vec!(),

        }
    }
}



pub struct ProgramTextures<'a> {
    pub ground: Texture<'a>,
}

impl Debug for ProgramTextures<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "[program's textures]")
    }
}



#[derive(Debug)]
pub struct ProgramSettings {
    pub last_open_files: Vec<String>,
}



#[derive(Debug)]
pub enum ProgramTask {
    LoadFile (String)
}



#[derive(Debug)]
pub struct File {
    pub path: String,
    pub contents: Vec<String>,
}





pub enum TaskUpdateInfo {
    AddFile (File)
}
