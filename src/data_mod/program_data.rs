use std::cell::RefCell;

use crate::prelude::*;
use sdl2::{render::Texture};





#[derive(fmt_derive::Debug, Clone, SmartDefault)]
pub struct ProgramData {

    pub settings: Shared<Option<ProgramSettings>>,
    pub errors: Shared<Vec<Error>>,
    pub tasks: Shared<Vec<ProgramTask>>,

    pub frame_count: Shared<u32>, // overflows after ~10,000 hours at 120 fps
      #[default(Instant::now())]
    pub start_instant: Instant,
      #[default(Shared::take(Instant::now()))]
    pub last_frame_instant: Shared<Instant>,
    pub exit: Shared<bool>,

    pub keys_pressed: Shared<KeysPressed>,
    pub last_text_input_timestamp: Shared<u32>,
    
    pub files: Shared<Vec<File>>,
    pub current_file_num: Shared<Option<usize>>,
      #[default(Shared::take(Instant::now()))]
    pub cursor_place_instant: Shared<Instant>,

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





#[derive(Debug, Default)]
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
}
