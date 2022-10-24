use crate::{data::{errors::*, errors::Result::*}, fns};

use std::{
    fmt::{self, Debug},
    sync::{Arc, Mutex}
};
use sdl2::{render::Texture};



#[derive(Debug)]
pub struct ProgramData {

    pub frame_count: Shared<u32>, // overflows after ~10,000 hours at 120 fps
    pub exit: Shared<bool>,
    pub settings: Shared<Option<ProgramSettings>>,

    pub tasks: Shared<Vec<ProgramTask>>,
    pub errors: Shared<Vec<Error>>,

    pub open_files: Shared<Vec<File>>,

}

impl ProgramData {
    pub fn new() -> Self {
        Self {

            frame_count: Shared::take(0),
            exit: Shared::take(false),
            settings: Shared::take(None),

            tasks: Shared::take(vec!()),
            errors: Shared::take(vec!()),

            open_files: Shared::take(vec!()),

        }
    }
    pub fn clone (&self) -> Self {
        Self {

            frame_count: self.frame_count.clone(),
            exit: self.exit.clone(),
            settings: self.settings.clone(),

            tasks: self.tasks.clone(),
            errors: self.errors.clone(),

            open_files: self.open_files.clone(),

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





pub type Shared<T> = Arc<Mutex<T>>;

pub trait SharedFns<T> {
    fn take (v: T) -> Self;
}

impl<T> SharedFns<T> for Shared<T> {
    fn take (v: T) -> Self {
        Arc::new(Mutex::new(v))
    }
}
