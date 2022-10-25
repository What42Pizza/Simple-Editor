use serde_hjson::{Map, Value};

use crate::{data::{program_data::*, errors::*, errors::Result::*}, fns};



#[derive(Debug)]
pub struct ProgramSettings {

    pub background_color: u32,

    pub continue_details: ContinueDetails,

}

impl ProgramSettings {
    pub fn default() -> Self {
        Self {

            background_color: 0x1B212F,

            continue_details: ContinueDetails {
                last_open_files: vec!(),
            },

        }
    }
}



#[derive(Debug)]
pub struct ContinueDetails {
    pub last_open_files: Vec<String>,
}





pub fn get_setting<T> (settings: &Map<String, Value>, full_key: &str, value_fn: impl FnOnce(&Value) -> Option<T>, value_type_name: &str, default_value: T) -> T {

    let found_value = match fns::get_hjson_value(settings, full_key) {
        Some(v) => v,
        None => {
            println!("Warning: could not find setting \"{}\"", full_key);
            return default_value;
        }
    };

    match value_fn(found_value) {
        Some(v) => v,
        None => {
            println!("Warning: setting \"{}\" needs to be of type {}, but was found to be of type {}", full_key, value_type_name, fns::get_value_type_name(found_value));
            default_value
        }
    }

}



pub fn get_setting_defaultless<T> (settings: &Map<String, Value>, full_key: &str, value_fn: impl FnOnce(&Value) -> Option<T>, value_type_name: &str) -> Option<T> {

    let found_value = match fns::get_hjson_value(settings, full_key) {
        Some(v) => v,
        None => {
            println!("Warning: could not find setting \"{}\"", full_key);
            return None;
        }
    };

    match value_fn(found_value) {
        Some(v) => Some(v),
        None => {
            println!("Warning: setting \"{}\" needs to be of type {}, but was found to be of type {}", full_key, value_type_name, fns::get_value_type_name(found_value));
            return None;
        }
    }

}





pub fn get_setting_string_array (settings: &Map<String, Value>, full_key: &str, default_value: Vec<String>) -> Vec<String> {
    // supposedly, in the future you could use lifetimes instead of this closure (`::<&'a Vec<Value>>`)
    match get_setting_defaultless::<Vec<Value>>(settings, full_key, |value| value.as_array().cloned(), "Array") {
        Some(value) => value.iter()
            .filter_map(|value|
                value.as_str().map(|s| s.to_string())
            )
            .collect(),
        None => default_value
    }
}



pub fn get_setting_color (settings: &Map<String, Value>, full_key: &str, default_value: u32) -> u32 {
    match get_setting_defaultless(settings, full_key, Value::as_u64, "U64") {
        Some(value) => fns::u64_to_color(value).unwrap_or(default_value),
        None => default_value
    }
}
