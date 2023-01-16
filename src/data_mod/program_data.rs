use std::cell::RefCell;

use crate::prelude::*;
use sdl2::{render::Texture};





type FrameUpdateFns = Box<dyn FnOnce(&ProgramData, &Duration) + Send + Sync>;

#[derive(fmt_derive::Debug)]
pub struct ProgramData {

    pub settings: Shared<Option<ProgramSettings>>,
    pub errors: Shared<Vec<Error>>,
    pub frame_count: Shared<u32>, // overflows after ~10,000 hours at 120 fps
    pub start_instant: Instant,
    pub last_frame_instant: Shared<Instant>,
    pub exit: Shared<bool>,

    pub keys_pressed: Shared<KeysPressed>,
    pub last_text_input_timestamp: Shared<u32>,
    
    pub tasks: Shared<Vec<ProgramTask>>,

    pub files: Shared<Vec<File>>,
    pub current_file_num: Shared<Option<usize>>,
    pub cursor_place_instant: Shared<Instant>,

}

impl ProgramData {
    pub fn new() -> Self {
        Self {

            settings: Shared::take(None),
            errors: Shared::take(vec!()),
            frame_count: Shared::take(0),
            start_instant: Instant::now(),
            last_frame_instant: Shared::take(Instant::now()),
            exit: Shared::take(false),

            keys_pressed: Shared::take(KeysPressed::new()),
            last_text_input_timestamp: Shared::take(0),
            
            tasks: Shared::take(vec!()),

            files: Shared::take(vec!()),
            current_file_num: Shared::take(None),
            cursor_place_instant: Shared::take(Instant::now()),

        }
    }
    pub fn clone (&self) -> Self {
        Self {

            settings: self.settings.clone(),
            errors: self.errors.clone(),
            frame_count: self.frame_count.clone(),
            start_instant: self.start_instant,
            last_frame_instant: self.last_frame_instant.clone(),
            exit: self.exit.clone(),

            keys_pressed: self.keys_pressed.clone(),
            last_text_input_timestamp: self.last_text_input_timestamp.clone(),
            
            tasks: self.tasks.clone(),

            files: self.files.clone(),
            current_file_num: self.current_file_num.clone(),
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
                    selection_start: None,
                }
            ],
        }
    }
}



#[derive(Debug)]
pub struct Cursor {
    pub x: usize,
    pub y: usize,
    pub wanted_x: usize,
    pub selection_start: Option<(usize, usize)>,
}





#[derive(Debug)]
pub struct KeysPressed {
    pub shift_pressed: bool,
    pub control_pressed: bool,
    pub alt_pressed: bool,
}

impl KeysPressed {
    pub fn new() -> Self {
        Self {
            shift_pressed: false,
            control_pressed: false,
            alt_pressed: false,
        }
    }
}





pub type Shared<T> = Arc<AtomicRefCell<T>>;

pub trait SharedFns<T> {
    fn take (v: T) -> Self;
}

impl<T> SharedFns<T> for Shared<T> {
    fn take (v: T) -> Self {
        Arc::new(AtomicRefCell::new(v))
    }
}





#[derive(Debug)]
pub enum ProgramTask {
    LoadFile {file_path: String, switch_to_this: bool},
    SaveFile {file_num: usize, file_path: String},
    CloseFile {file_num: usize},
}
