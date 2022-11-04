use crate::{data::{program_data::*, settings::*, errors::*, errors::Result::*}};

use std::{path::PathBuf, fs::OpenOptions};
use sdl2::{rect::Rect, pixels::Color, surface::Surface, video::WindowContext,
    render::{Texture, TextureCreator}
};
use serde_hjson::{Value, Map};
use regex::Regex;
use lerp::Lerp;



pub fn get_texture_size (texture: &Texture) -> (u32, u32) {
    let query = texture.query();
    (query.width, query.height)
}

pub fn get_spritesheet_src_from_index (spritesheet: &Texture, index: u32, sprite_width: u32, sprite_height: u32) -> Rect {
    let spritesheet_width = spritesheet.query().width;
    let sprites_per_row = spritesheet_width / sprite_width;
    let row_num = index % sprites_per_row;
    let column_num = index / sprites_per_row;
    Rect::new((row_num * sprite_width) as i32, (column_num * sprite_height) as i32, sprite_width, sprite_height)
}



pub fn get_program_dir() -> PathBuf {
    let mut buf = std::env::current_exe()
        .expect("Could not retrieve the path for the current exe.");
    buf.pop();
    buf
}



pub fn get_file_exists (path: &PathBuf) -> Result<bool> {
    let file = OpenOptions::new().read(true).open(path);
    if file.is_ok() {return Ok(true);}
    let err = file.unwrap_err();
    match err.kind() {
        std::io::ErrorKind::NotFound => Ok(false),
        _ => Err(err.into()),
    }
}



pub fn split_lines (full_str: &str) -> Vec<String> {
    Regex::new("((\r\n)|\n)").unwrap().split(full_str)
        .map(|s| s.to_string())
        .collect()
}



pub fn some_if<T> (condition: bool, some_fn: impl FnOnce() -> T) -> Option<T> {
    if condition {
        Some(some_fn())
    } else {
        None
    }
}



pub fn u64_to_color_rgb (input: u64) -> Option<Color> {
    some_if(input <= 0xFFFFFF, || {
        Color::RGB(
            get_byte(input, 2),
            get_byte(input, 1),
            get_byte(input, 0),
        )
    })
}

pub fn get_byte (input: u64, byte_num: u8) -> u8 {
    ((input & (0xFF << (byte_num * 8))) >> (byte_num * 8)) as u8
}



pub fn blend_colors (color1: Color, color2: Color, blend_amount: f64) -> Color {
    let (r1, g1, b1) = color1.rgb();
    let (r2, g2, b2) = color2.rgb();
    let r = (r1 as f64).lerp(r2 as f64, blend_amount) as u8;
    let g = (g1 as f64).lerp(g2 as f64, blend_amount) as u8;
    let b = (b1 as f64).lerp(b2 as f64, blend_amount) as u8;
    Color::RGB(r, g, b)
}



pub fn get_empty_texture (texture_creator: &TextureCreator<WindowContext>) -> Result<Texture<'_>> {
    texture_creator
        .create_texture_from_surface(Surface::new(1, 1, sdl2::pixels::PixelFormatEnum::ARGB8888).unwrap())
        .to_custom_err()
}



pub fn get_value_type_name (value: &Value) -> String {
    String::from(match value {
        Value::Null => "Null",
        Value::Bool(_) => "Bool",
        Value::I64(_) => "I64",
        Value::U64(_) => "U64",
        Value::F64(_) => "F64",
        Value::String(_) => "String",
        Value::Array(_) => "Array",
        Value::Object(_) => "Object",
    })
}



pub fn get_hjson_value<'a> (starting_object: &'a Map<String, Value>, full_key: &str) -> Option<&'a Value> {

    let mut current_object = starting_object;
    let keys = full_key.split('/').collect::<Vec<&str>>();
    for current_key in keys.iter().take(keys.len() - 1) {
        let next_object = match current_object.get(&current_key.to_string()) {
            Some(v) => v,
            None => {return None;}
        };
        current_object = match next_object {
            Value::Object(v) => v,
            _ => {return None;}
        };
    }

    current_object.get(keys[keys.len()-1])

}
