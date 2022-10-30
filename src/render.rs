use std::sync::MutexGuard;

use crate::{data::{program_data::*, settings::*, errors::*, errors::Result::*}, fns};

use sdl2::{render::{WindowCanvas, TextureCreator}, video::WindowContext, ttf::Font, pixels::Color, rect::Rect};



pub fn render(canvas: &mut WindowCanvas, program_data: &ProgramData, textures: &ProgramTextures<'_>, texture_creator: &TextureCreator<WindowContext>, font: &Font) -> Result<()> {

    // get data
    let settings_mutex = program_data.settings.lock().unwrap();
    let settings = settings_mutex.as_ref().expect("Error: settings is none");
    let (width, height) = canvas.output_size().to_custom_err()?;

    // clear
    canvas.set_draw_color(settings.background_color);
    canvas.clear();

    // render
    let files = program_data.files.lock().unwrap();
    let current_file = match get_file_to_render(program_data, &files) {
        Some(v) => v,
        None => {return Ok(());}
    };

    for (i, current_line) in current_file.contents.iter().enumerate() {
        render_text_at_line(current_line, (i as i64 * settings.font_size) as i32, font, canvas, texture_creator)?;
    }

    // finish
    canvas.present();
    Ok(())

}





pub fn get_file_to_render<'a> (program_data: &ProgramData, files: &'a MutexGuard<Vec<File>>) -> Option<&'a File> {

    // select file
    let current_file = program_data.current_file.lock().unwrap();
    if current_file.is_none() {return None;}
    let current_file = current_file.unwrap();

    // get contents
    if current_file >= files.len() {
        program_data.errors.lock().unwrap().push(Error::new("InvalidFileNum", &(
            if files.len() == 0 {
                "Current file num is ".to_string() + &current_file.to_string() + " but there no files open"
            } else if files.len() == 1 {
                "Current file num is ".to_string() + &current_file.to_string() + " but there is only 1 file open"
            } else {
                "Current file num is ".to_string() + &current_file.to_string() + " but there are only " + &files.len().to_string() + " files open"
            }
        )));
        return None;
    }

    Some(&files[current_file])

}



pub fn render_text_at_line (text: &str, y: i32, font: &Font, canvas: &mut WindowCanvas, texture_creator: &TextureCreator<WindowContext>) -> Result<()> {

    let text_surface = font
        .render(text)
        .blended(Color::RGB(255, 255, 255))
        .to_custom_err()?;
    let text_texture = texture_creator
        .create_texture_from_surface(text_surface)
        .to_custom_err()?;
    let (width, height) = fns::get_texture_size(&text_texture);
    canvas.copy(&text_texture, None, Rect::new(0, y, width, height)).to_custom_err()?;

    Ok(())
}
