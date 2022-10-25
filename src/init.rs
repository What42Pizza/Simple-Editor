use crate::{data::{program_data::*, settings::*, errors::*, errors::Result::*}, fns};

use std::{fs, path::PathBuf};
use serde::__private::de;
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

    continue_session(program_data)?;

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

    let default_settings = ProgramSettings::default();

    let raw_settings = match
        load_raw_settings()
            .err_details("Could not load existing settings or default settings")?
    {
        Some(v) => v,
        None => {
            println!("Warning: no settings file found, using default settings");
            return Ok(default_settings);
        }
    };

    process_settings(&*raw_settings, &default_settings).or_else(|e| {
        println!("Warning: could not deserialize existing settings. Loading default settings...");
        println!("Error: {:#?}", e);
        Ok(default_settings)
    })

}



fn process_settings (raw_settings: &str, default_settings: &ProgramSettings) -> Result<ProgramSettings> {
    let settings = serde_hjson::from_str(raw_settings).to_custom_err()?;
    let settings = update_settings(settings)?;
    let settings = get_settings_from_hjson(settings, default_settings)?;
    Ok(settings)
}



pub fn load_raw_settings() -> Result<Option<String>> {

    let mut settings_path = fns::get_program_dir();
    settings_path.push("settings.txt");
    if !fns::get_file_exists(&settings_path)
        .err_details("Could not query location of settings file")
        .err_details_lazy(|| "  Path: ".to_string() + &settings_path.as_path().to_string_lossy())?
    {
        return Ok(None);
    }

    let raw_settings = fs::read_to_string(&settings_path)
        .err_details("Could not read settings file")
        .err_details_lazy(|| "  Path: ".to_string() + &settings_path.as_path().to_string_lossy())?;

    Ok(Some(raw_settings))

}



pub fn update_settings (mut settings: Value) -> Result<Map<String, Value>> {
    
    if !settings.is_object() {return err("LoadSettingsError", "Settings file is not an hjson object");}
    let mut settings: &mut Map<String, Value> = &mut settings.as_object().unwrap().to_owned();

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





fn get_settings_from_hjson (settings: Map<String, Value>, default_settings: &ProgramSettings) -> Result<ProgramSettings> {
    Ok(ProgramSettings {

        background_color: get_setting_color(&settings, "background color", default_settings.background_color),

        continue_details: ContinueDetails {
            last_open_files: get_setting_string_array(&settings, "continue details/last open files", vec!()),
        },

    })
}





pub fn continue_session (program_data: &mut ProgramData) -> Result<()> {

    let settings = program_data.settings.lock().unwrap();
    let continue_details = &settings.none_err("ContinueSessionError", "Settings is none")?.continue_details;
    let mut tasks = program_data.tasks.lock().unwrap();

    for file_path in &continue_details.last_open_files {
        tasks.push(ProgramTask::LoadFile(file_path.to_string()));
    }

    Ok(())
}





type SettingsUpdaterFn = dyn Fn(&mut Map<String, Value>);

const SETTINGS_UPDATER_FNS: [&SettingsUpdaterFn; 1] = [
    /* 0 */ &|_| {
        println!("Settings are up to date");
    },
];
