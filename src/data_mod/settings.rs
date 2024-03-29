use crate::prelude::*;
use sdl2::pixels::Color;



#[derive(Debug)]
pub struct ProgramSettings {

    pub frame_timing: FrameTimingSetting,
    pub background_color: Color,

    pub font_path: String,
    pub font_size: u32,
    pub font_spacing: f64,

    pub cursor_flashing_speed: f64,
    pub cursor_width: f64, // relative to screen width
    pub cursor_height: f64, // relative to font height
    pub cursor_color: Color,
    pub cursor_selection_color: Color,

    pub continue_details: ContinueDetails,

}

impl Default for ProgramSettings {
    fn default() -> Self {
        Self {

            frame_timing: FrameTimingSetting::VSync,
            background_color: Color::RGB(27, 33, 47),

            font_path: String::from(""),
            font_size: 32,
            font_spacing: 1.1,

            cursor_flashing_speed: 1.0,
            cursor_width: 1./500.,
            cursor_height: 1.1,
            cursor_color: Color::RGB(255, 255, 255),
            cursor_selection_color: Color::RGBA(0, 31, 255, 127),

            continue_details: ContinueDetails {
                last_open_files: vec!(),
            },

        }
    }
}



pub trait SettingValueEnum {
    fn get_type(&self) -> String;
}



#[derive(Debug)]
pub struct ContinueDetails {
    pub last_open_files: Vec<String>,
}



#[derive(Debug, Clone, Copy)]
pub enum FrameTimingSetting {
    VSync,
    Maxxed (usize),
    None,
}

impl SettingValueEnum for FrameTimingSetting {
    fn get_type(&self) -> String {
        match self {
            Self::VSync => String::from("VSync"),
            Self::Maxxed(_) => String::from("Maxxed"),
            Self::None => String::from("None"),
        }
    }
}





type SettingsUpdaterFn = dyn Fn(&mut Map<String, Value>);

const SETTINGS_UPDATER_FNS: [&SettingsUpdaterFn; 1] = [
    /* 0 */ &|_| {
        println!("Settings are up to date");
    },
];






pub fn load_settings() -> ProgramSettings {

    let default_settings = ProgramSettings::default();

    let raw_settings = match load_raw_settings() {
        Ok(v) => v,
        Err(error) => {
            println!("Warning: no settings file found, loading default settings...");
            println!("Error: {:?}", error);
            return default_settings;
        }
    };

    let raw_settings = match raw_settings {
        Some(v) => v,
        None => {
            println!("Warning: no settings file found, loading default settings...");
            return default_settings;
        }
    };

    process_settings(&raw_settings, &default_settings).unwrap_or_else(|e| {
        println!("Warning: could not deserialize existing settings, loading default settings...");
        println!("Error: {:#?}", e);
        default_settings
    })

}



fn process_settings (raw_settings: &str, default_settings: &ProgramSettings) -> Result<ProgramSettings, ProgramError> {
    let settings = serde_hjson::from_str(raw_settings)?;
    let settings = update_settings(settings)?;
    let settings = get_settings_from_hjson(settings, default_settings)?;
    Ok(settings)
}



pub fn load_raw_settings() -> Result<Option<String>, ProgramError> {

    let mut settings_path = fns::get_program_dir();
    settings_path.push("settings.hjson");
    match fns::get_file_exists(&settings_path) {
        Ok(false) => return Ok(None),
        Ok(true) => {},
        Err(error) => return err(RawProgramError::CouldNotLoadFile {
            file_path: settings_path.to_string_lossy().to_string(),
            source: error,
        }),
    };

    let raw_settings = match fs::read_to_string(&settings_path) {
        Ok(v) => v,
        Err(error) => return err(RawProgramError::CouldNotLoadFile {
            file_path: settings_path.to_string_lossy().to_string(),
            source: error,
        }),
    };

    Ok(Some(raw_settings))

}



pub fn update_settings (settings: Value) -> Result<Map<String, Value>, ProgramError> {
    
    if !settings.is_object() {return err(RawProgramError::SettingsAreNotAnObject);}
    let settings: &mut Map<String, Value> = &mut settings.as_object().unwrap().to_owned();

    let settings_version = match get_setting_defaultless(settings, "settings version", Value::as_u64, "u64") {
        Some(v) => v,
        None => {
            println!("Warning: could not get settings version, settings will not be updated");
            return Ok(settings.to_owned());
        }
    } as usize;

    if settings_version >= SETTINGS_UPDATER_FNS.len() {
        println!("Warning: settings version is invalid (greater than most recent settings version ({}))", SETTINGS_UPDATER_FNS.len() - 1);
        return Ok(settings.to_owned());
    }

    for updater_fn in SETTINGS_UPDATER_FNS.iter().skip(settings_version) {
        updater_fn(settings);
    }

    Ok(settings.to_owned())
}





fn get_settings_from_hjson (settings: Map<String, Value>, default_settings: &ProgramSettings) -> Result<ProgramSettings, ProgramError> {

    let frame_timing = match get_setting_borrowed(&settings, "frame timing/type", Value::as_str, "string", &default_settings.frame_timing.get_type()) {
        "vsync" => FrameTimingSetting::VSync,
        "maxxed" => {
            let mut max_frame_time = get_setting(&settings, "frame timing/max frame time", Value::as_i64, "i64", 5);
            if max_frame_time < 0 {
                println!("Warning: invalid setting value for \"frame timing/max frame time\", cannot be below 0");
                max_frame_time = 5;
            }
            FrameTimingSetting::Maxxed(max_frame_time as usize)
        }
        "none" => FrameTimingSetting::None,
        _ => {
            println!("Warning: invalid setting value for \"frame timing/type\", only \"vsync\", \"maxxed\", and \"none\" are allowed");
            default_settings.frame_timing
        }
    };

    Ok(ProgramSettings {

        frame_timing,
        background_color: get_setting_color(&settings, "background color", default_settings.background_color),

        font_path: get_setting_lazy(&settings, "font path", |v| v.as_str().map(str::to_string), "String", || default_settings.font_path.to_string()),
        font_size: get_setting(&settings, "font size", Value::as_i64, "i64", default_settings.font_size as i64) as u32,
        font_spacing: get_setting(&settings, "font spacing", Value::as_f64, "f64", default_settings.font_spacing),

        cursor_flashing_speed: get_setting(&settings, "cursor flashing speed", Value::as_f64, "f64", default_settings.cursor_flashing_speed),
        cursor_width: get_setting(&settings, "cursor width", Value::as_f64, "f64", default_settings.cursor_width),
        cursor_height: get_setting(&settings, "cursor height", Value::as_f64, "f64", default_settings.cursor_height),
        cursor_color: get_setting_color(&settings, "cursor color", default_settings.cursor_color),
        cursor_selection_color: get_setting_color(&settings, "cursor selection color", default_settings.cursor_selection_color),

        continue_details: ContinueDetails {
            last_open_files: get_setting_string_array(&settings, "continue details/last open files", vec!()),
        },

    })
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

pub fn get_setting_borrowed<'a, T: ?Sized> (settings: &'a Map<String, Value>, full_key: &str, value_fn: impl FnOnce(&Value) -> Option<&T>, value_type_name: &str, default_value: &'a T) -> &'a T {

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

pub fn get_setting_lazy<T> (settings: &Map<String, Value>, full_key: &str, value_fn: impl FnOnce(&Value) -> Option<T>, value_type_name: &str, default_value_fn: impl FnOnce() -> T) -> T {

    let found_value = match fns::get_hjson_value(settings, full_key) {
        Some(v) => v,
        None => {
            println!("Warning: could not find setting \"{}\"", full_key);
            return default_value_fn();
        }
    };

    match value_fn(found_value) {
        Some(v) => v,
        None => {
            println!("Warning: setting \"{}\" needs to be of type {}, but was found to be of type {}", full_key, value_type_name, fns::get_value_type_name(found_value));
            default_value_fn()
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
            None
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



pub fn get_setting_color (settings: &Map<String, Value>, full_key: &str, default_value: Color) -> Color {
    match get_setting_defaultless(settings, full_key, Value::as_u64, "U64") {
        Some(value) => fns::u64_to_color(value).unwrap_or(default_value),
        None => default_value
    }
}
