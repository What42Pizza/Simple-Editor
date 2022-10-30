use crate::{data::{program_data::*, settings::*, errors::*, errors::Result::*}, fns};

use sdl2::{Sdl, pixels::Color,
    image::{self, LoadTexture, InitFlag},
    render::{Canvas, TextureCreator, Texture},
    video::{Window, WindowContext}, ttf::{Sdl2TtfContext, Font}
};





pub fn init_sdl2() -> (Sdl, Sdl2TtfContext, Canvas<Window>) {

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

    let ttf_context = sdl2::ttf::init().expect("Could not initialize sdl2::ttf");

    (sdl_context, ttf_context, canvas)

}





pub fn init_program_data<'a> (program_data: &mut ProgramData, texture_creator: &'a TextureCreator<WindowContext>, ttf_context: &'a Sdl2TtfContext) -> Result<(Font<'a, 'a>, ProgramTextures<'a>)> {

    let textures = load_textures(texture_creator)?;
    let settings = load_settings();

    let mut font_path = fns::get_program_dir();
    font_path.push(&settings.font_path);
    let font = ttf_context.load_font(font_path, settings.font_size as u16).to_custom_err()?;

    program_data.settings = Shared::take(Some(settings));
    continue_session(program_data)?;

    Ok((font, textures))

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





pub fn continue_session (program_data: &mut ProgramData) -> Result<()> {

    let settings = program_data.settings.lock().unwrap();
    let continue_details = &settings.none_err("ContinueSessionError", "Settings is none")?.continue_details;

    let mut tasks = program_data.tasks.lock().unwrap();
    for file_path in &continue_details.last_open_files {
        tasks.push(ProgramTask::LoadFile(file_path.to_string()));
    }
    drop(tasks);

    Ok(())
}