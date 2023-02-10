use crate::prelude::*;
use sdl2::{Sdl, pixels::Color,
    image::{self, InitFlag},
    render::{Canvas, TextureCreator, Texture},
    video::{Window, WindowContext},
    ttf::{Sdl2TtfContext, Font}
};





pub fn init_sdl2 (settings: &ProgramSettings) -> (Sdl, Sdl2TtfContext, Canvas<Window>) {

    let sdl_context = sdl2::init().expect("Could not initialize sdl2");
    let _image_context = image::init(InitFlag::PNG).expect("Could not retrieve sdl image context");
    let video_subsystem = sdl_context.video().expect("Could not retrieve video subsystem");
    let window = video_subsystem.window("SDL2 Testing Window", 1280, 720)
        .position_centered()
        .build()
        .expect("Could not build window");

    let mut canvas_builder = window.into_canvas()
        .accelerated();
    if let FrameTimingSetting::VSync = settings.frame_timing {canvas_builder = canvas_builder.present_vsync();}
    let mut canvas = canvas_builder
        .build()
        .expect("Could not build canvas");

    canvas.set_draw_color(Color::RGB(255, 0, 255));
    canvas.clear();
    canvas.present();

    let ttf_context = sdl2::ttf::init().expect("Could not initialize sdl2::ttf");

    (sdl_context, ttf_context, canvas)

}





pub fn init_program_data<'a> (program_data: &ProgramData, texture_creator: &'a TextureCreator<WindowContext>, ttf_context: &'a Sdl2TtfContext) -> Result<(Font<'a, 'a>, ProgramTextures<'a>), ProgramError> {
    let settings_mutex = program_data.settings.read();
    let settings = settings_mutex.as_ref().unwrap();

    let mut font_path = fns::get_program_dir();
    font_path.push(&settings.font_path);
    let font = ttf_context.load_font(font_path, settings.font_size as u16)?;

    let textures = load_textures(&font, texture_creator)?;

    drop(settings_mutex);
    continue_session(program_data)?;

    Ok((font, textures))

}



pub fn load_textures<'a> (font: &Font, texture_creator: &'a TextureCreator<WindowContext>) -> Result<ProgramTextures<'a>, ProgramError> {

    // render chars
    let mut ascii_chars = vec!();
    for i in (0..256).rev() {
        ascii_chars.push(render_char(char::from_u32(i).unwrap(), font, texture_creator)?);
    }
    let ascii_chars: [Texture; 256] = array_init::array_init(|_| ascii_chars.pop().unwrap());

    Ok(ProgramTextures {
        ascii_chars,
    })
}



pub fn render_char<'a> (char: char, font: &Font, texture_creator: &'a TextureCreator<WindowContext>) -> Result<Texture<'a>, ProgramError> {
    if char as u32 == 0 {
        return fns::get_empty_texture(texture_creator);
    }
    let char_surface = match font
        .render(&char.to_string())
        .blended(Color::RGB(255, 255, 255))
    {
        stdResult::Ok(v) => v,
        stdResult::Err(_) => return fns::get_empty_texture(texture_creator),
    };
    texture_creator.create_texture_from_surface(char_surface).map_err(|e| e.into())
}





pub fn continue_session (program_data: &ProgramData) -> Result<(), ProgramError> {

    let settings = program_data.settings.read();
    let continue_details = &settings.as_ref().expect("Settings cannot be None when calling 'init::continue_session'").continue_details;

    let mut tasks = program_data.tasks.write();
    for file_path in &continue_details.last_open_files {
        tasks.push(ProgramTask::LoadFile{file_path: file_path.to_string(), switch_to_this: true});
    }
    drop(tasks);

    Ok(())
}