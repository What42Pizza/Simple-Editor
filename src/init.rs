use crate::{data::{program_data::*, settings::*, errors::*, errors::Result::*}, fns};

use std::{sync::mpsc::Sender};
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





pub fn init_program_data<'a> (program_data: &mut ProgramData, texture_creator: &'a TextureCreator<WindowContext>, tasks_tx: &Sender<ProgramTask>) -> Result<ProgramTextures<'a>> {

    let textures = load_textures(texture_creator)?;
    program_data.settings = Shared::take(Some(load_settings()));

    continue_session(program_data, tasks_tx)?;

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





pub fn continue_session (program_data: &mut ProgramData, tasks_tx: &Sender<ProgramTask>) -> Result<()> {

    let settings = program_data.settings.lock().unwrap();
    let continue_details = &settings.none_err("ContinueSessionError", "Settings is none")?.continue_details;

    for file_path in &continue_details.last_open_files {
        let _ = tasks_tx.send(ProgramTask::LoadFile(file_path.to_string()));
    }

    Ok(())
}