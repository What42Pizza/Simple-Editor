use crate::{data::{program_data::*, errors::*, errors::Result::*}, fns};

use std::{fs, path::PathBuf};
use serde_hjson::{Map, Value};

use sdl2::{Sdl, pixels::Color,
    image::{self, LoadTexture, InitFlag},
    render::{Canvas, TextureCreator, Texture},
    video::{Window, WindowContext}
};





pub fn init_sdl2() -> (Sdl, Canvas<Window>) {
    
    let sdl_context = sdl2::init().expect("Could not initialize sdl2");
    let _image_context = image::init(InitFlag::PNG).expect("Could not retrieve sdl image context");
    let video_subsystem = sdl_context.video().expect("Could not retrieve video subsystem");
    let window = video_subsystem.window("SDL2 Testing Window", 1280, 720)
        .position_centered()
        .build()
        .expect("Could not build window");
    
    let mut canvas = window.into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .expect("Could not build canvas");

    canvas.set_draw_color(Color::RGB(255, 0, 255));
    canvas.clear();
    canvas.present();

    (sdl_context, canvas)

}





pub fn init_program_data<'a> (program_data: &mut ProgramData, texture_creator: &'a TextureCreator<WindowContext>) -> Result<ProgramTextures<'a>> {

    let textures = load_textures(texture_creator)?;
    program_data.settings = Shared::take(Some(load_settings()?));

    Ok(textures)

}



pub fn load_textures (texture_creator: &TextureCreator<WindowContext>) -> Result<ProgramTextures<'_>> {
    Ok(ProgramTextures {
        ground: load_texture("assets/ground.png", texture_creator)?,
    })
}

pub fn load_texture<'a> (texture_name: &str, texture_creator: &'a TextureCreator<WindowContext>) -> Result<Texture<'a>> {
    Ok(texture_creator.load_texture(texture_name)
        .err_details_lazy(|| ("  Texture: \"".to_string() + texture_name + "\""))?
    )
}



pub fn load_settings() -> Result<ProgramSettings> {

    let raw_settings = &*load_raw_settings()
        .err_details("Could not load existing settings or default settings")?;
    
    match process_settings(raw_settings) {
        Ok(v) => Ok(v),
        Err(e) => {
            println!("Warning: could not deserialize existing settings. Loading default settings...");
            println!("Error: {:#?}", e);
            process_settings(include_str!("default_settings.hjson"))
        }
    }

}



fn process_settings (raw_settings: &str) -> Result<ProgramSettings> {
    let settings = serde_hjson::from_str(raw_settings).to_custom_err()?;
    let settings = update_settings(settings)?;
    let settings = get_settings_from_hjson(settings)?;
    Ok(settings)
}



pub fn load_raw_settings() -> Result<String> {

    let mut settings_path = fns::get_program_dir();
    settings_path.push("settings.txt");
    if !fns::get_file_exists(&settings_path)
        .err_details("Could not query location of settings file")
        .err_details_lazy(|| "  Path: ".to_string() + &settings_path.as_path().to_string_lossy())?
    {
        create_default_settings_file(&settings_path)?;
    }

    let raw_settings = fs::read_to_string(&settings_path)
        .err_details("Could not read settings file")
        .err_details_lazy(|| "  Path: ".to_string() + &settings_path.as_path().to_string_lossy())?;

    Ok(raw_settings)

}



pub fn create_default_settings_file (path: &PathBuf) -> Result<()> {
    fs::write(path, include_str!("default_settings.hjson"))
        .err_details("Failed to create default settings file")
        .err_details_lazy(|| "  Path: ".to_string() + &path.as_path().to_string_lossy())
}



pub fn update_settings (mut settings: Value) -> Result<Map<String, Value>> {
    
    if !settings.is_object() {return err("LoadSettingsError", "Settings file is not an hjson object");}
    let mut settings: &mut Map<String, Value> = &mut settings.as_object().unwrap().to_owned();

    let settings_version = match settings.get("settings version") {
        Some(settings_version) => settings_version,
        None => {
            println!("Warning: settings version could not be found");
            return Ok(settings.to_owned());
        }
    };

    let settings_version = match settings_version.as_u64() {
        Some(settings_version) => settings_version,
        None => {
            println!("Warning: settings version is not a positive integer");
            return Ok(settings.to_owned());
        }
    } as usize;

    if settings_version >= SETTINGS_UPDATER_FNS.len() {
        println!("Warning: settings version is invalid (greater than most recent settings version ({}))", SETTINGS_UPDATER_FNS.len() - 1);
    }

    for updater_fn in SETTINGS_UPDATER_FNS.iter().skip(settings_version) {
        updater_fn(settings);
    }

    Ok(settings.to_owned())
}





fn get_settings_from_hjson (settings: Map<String, Value>) -> Result<ProgramSettings> {

    let last_open_files = get_last_open_files(&settings);
    println!("{:#?}", last_open_files);

    Ok(ProgramSettings {
        last_open_files,
    })
}



fn get_last_open_files (settings: &Map<String, Value>) -> Vec<String> {

    let array = match fns::get_hjson_array(settings, "continue details/last open files"){
        Some(v) => v,
        None => {return vec!();}
    };

    let mut output = vec!();
    for current_value in array {
        if let Value::String(string) = current_value {
            output.push(string.to_string());
        }
    }

    output
}





type SettingsUpdaterFn = dyn Fn(&mut Map<String, Value>);

const SETTINGS_UPDATER_FNS: [&SettingsUpdaterFn; 1] = [
    /* 0 */ &|_| {
        println!("Settings are up to date");
    },
];
