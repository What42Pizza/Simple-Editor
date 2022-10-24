use crate::{data::{program_data::*, errors::*, errors::Result::*}};

use std::{path::PathBuf, fs::OpenOptions};
use sdl2::{render::Texture, rect::Rect};
use serde_hjson::{Value, Map};



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





pub fn get_hjson_array<'a> (starting_object: &'a Map<String, Value>, full_key: &str) -> Option<&'a Vec<Value>> {

    let (parent_object, key) = match get_hjson_parent_object (starting_object, full_key) {
        Some(v) => v,
        None => {
            println!("Warning: could not find setting \"{}\"", full_key);
            return None;
        }
    };

    let found_value = match parent_object.get(&*key) {
        Some(v) => v,
        None => {
            println!("Warning: could not find setting \"{}\"", full_key);
            return None;
        }
    };
    match found_value {
        Value::Array(v) => Some(v),
        _ => {
            println!("Warning: setting \"{}\" is not an array", full_key);
            return None;
        }
    }

}



pub fn get_hjson_parent_object<'a> (starting_object: &'a Map<String, Value>, key: &str) -> Option<(&'a Map<String, Value>, String)> {

    let mut current_object = starting_object;
    let keys = key.split("/").collect::<Vec<&str>>();
    for current_key in keys.iter().take(keys.len() - 1) {
        let next_object = match current_object.get(&current_key.to_string()) {
            Some(v) => v,
            None => {return None;}
        };
        current_object = match next_object {
            Value::Object(v) => &v,
            _ => {return None;}
        };
    }

    Some((current_object, keys[keys.len()-1].to_string()))

}
