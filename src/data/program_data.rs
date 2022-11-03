use crate::{data::{settings::*, errors::*, errors::Result::*}, fns};

use std::{
    fmt::{self, Debug},
    sync::{Arc, Mutex},
    time::{Instant, Duration}
};
use sdl2::{render::Texture, event::Event};





type FrameUpdateFns = Box<dyn FnOnce(&ProgramData, &Duration) + Send>;

#[derive(fmt_derive::Debug)]
pub struct ProgramData {

    pub settings: Shared<Option<ProgramSettings>>,
    pub errors: Shared<Vec<Error>>,
    pub frame_count: Shared<u32>, // overflows after ~10,000 hours at 120 fps
    pub start_instant: Instant,
    pub exit: Shared<bool>,
    
    pub tasks: Shared<Vec<ProgramTask>>,
    pub frame_update_fns: Shared<Vec<FrameUpdateFns>>,
    pub has_frame_update_task: Shared<bool>,
    pub last_frame_updates_time: Shared<Instant>,

    pub files: Shared<Vec<File>>,
    pub current_file: Shared<Option<usize>>,
    pub cursor_place_instant: Shared<Instant>,

}

impl ProgramData {
    pub fn new() -> Self {
        Self {

            settings: Shared::take(None),
            errors: Shared::take(vec!()),
            frame_count: Shared::take(0),
            start_instant: Instant::now(),
            exit: Shared::take(false),
            
            tasks: Shared::take(vec!()),
            frame_update_fns: Shared::take(vec!()),
            has_frame_update_task: Shared::take(false),
            last_frame_updates_time: Shared::take(Instant::now()),

            files: Shared::take(vec!()),
            current_file: Shared::take(None),
            cursor_place_instant: Shared::take(Instant::now()),

        }
    }
    pub fn clone (&self) -> Self {
        Self {

            settings: self.settings.clone(),
            errors: self.errors.clone(),
            frame_count: self.frame_count.clone(),
            start_instant: self.start_instant.clone(),
            exit: self.exit.clone(),
            
            tasks: self.tasks.clone(),
            frame_update_fns: self.frame_update_fns.clone(),
            has_frame_update_task: self.has_frame_update_task.clone(),
            last_frame_updates_time: self.last_frame_updates_time.clone(),

            files: self.files.clone(),
            current_file: self.current_file.clone(),
            cursor_place_instant: self.cursor_place_instant.clone(),

        }
    }
}



#[derive(fmt_derive::Debug)]
pub struct ProgramTextures<'a> {
    pub ascii_chars: [Texture<'a>; 256]
}



#[derive(Debug)]
pub struct File {
    pub path: String,
    pub contents: Vec<Vec<char>>,
    pub scroll_x: f64,
    pub scroll_y: f64,
    pub cursors: Vec<Cursor>,
    pub selection_start: Option<(usize, usize)>,
}

impl File {
    pub fn new (path: String, contents: Vec<String>) -> Self {
        Self {
            path,
            contents: contents.iter().map(|s| s.chars().collect()).collect(),
            scroll_x: 0.,
            scroll_y: 0.,
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
    DoFrameUpdates,
    LoadFile (String),
    SaveFile (String),
    CloseFile (String),
    HandleEvent (Event),
}
