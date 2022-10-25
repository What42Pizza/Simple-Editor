use crate::{data::{settings::*, errors::*, errors::Result::*}, fns};

use std::{result::Result as stdResult,
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

    pub files: Shared<Vec<File>>,

}

impl ProgramData {
    pub fn new() -> Self {
        Self {

            frame_count: Shared::take(0),
            exit: Shared::take(false),
            settings: Shared::take(None),

            tasks: Shared::take(vec!()),
            errors: Shared::take(vec!()),

            files: Shared::take(vec!()),

        }
    }
    pub fn clone (&self) -> Self {
        Self {

            frame_count: self.frame_count.clone(),
            exit: self.exit.clone(),
            settings: self.settings.clone(),

            tasks: self.tasks.clone(),
            errors: self.errors.clone(),

            files: self.files.clone(),

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
pub struct File {
    pub path: String,
    pub contents: Vec<String>,
    pub cursors: Vec<Cursor>,
    pub selection_start: Option<(usize, usize)>,
}

impl File {
    pub fn new (path: String, contents: Vec<String>) -> Self {
        Self {
            path: path,
            contents: contents,
            cursors: vec![
                Cursor {
                    x: 0,
                    y: 0,
                    wanted_x: 0,
                }
            ],
            selection_start: None,
        }
    }
}

#[derive(Debug)]
pub struct Cursor {
    pub x: usize,
    pub y: usize,
    pub wanted_x: usize,
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





#[derive(Debug)]
pub enum ProgramTask {
    LoadFile (String),
    SaveFile (String),
}
