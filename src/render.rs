use crate::{data::{program_data::*, settings::*, errors::*, errors::Result::*}, fns};

use std::sync::MutexGuard;
use sdl2::{render::{WindowCanvas, TextureCreator, Texture}, video::WindowContext, ttf::Font, pixels::Color, rect::Rect};



pub fn render(canvas: &mut WindowCanvas, program_data: &ProgramData, textures: &ProgramTextures<'_>, texture_creator: &TextureCreator<WindowContext>, font: &Font) -> Result<()> {

    // get data
    let settings_mutex = program_data.settings.lock().unwrap();
    let settings = settings_mutex.as_ref().expect("Error: settings is none");
    let (width, height) = canvas.output_size().to_custom_err()?;
    let buttons_bottom_y = div(height, 20.);

    // clear
    canvas.set_draw_color(settings.background_color);
    canvas.clear();

    // render
    let files = program_data.files.lock().unwrap();
    let current_file = match get_file_to_render(program_data, &files) {
        Some(v) => v,
        None => {return Ok(());}
    };

    let text_section = Rect::new(0, buttons_bottom_y as i32, width, height - buttons_bottom_y);
    let spacing = (settings.font_size as f64 * settings.font_spacing) as i32;
    for (i, current_line) in current_file.contents.iter().enumerate() {
        render_text(current_line, div(width, 75.) as i32, i as i32 * spacing, text_section, font, canvas, texture_creator)?;
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



pub fn render_text (text: &str, x:i32, y: i32, section: Rect, font: &Font, canvas: &mut WindowCanvas, texture_creator: &TextureCreator<WindowContext>) -> Result<()> {

    let text_surface = font
        .render(text)
        .blended(Color::RGB(255, 255, 255))
        .to_custom_err()?;
    let text_texture = texture_creator
        .create_texture_from_surface(text_surface)
        .to_custom_err()?;
    let (width, height) = fns::get_texture_size(&text_texture);

    //canvas.copy(&text_texture, None, Rect::new(x, y, width, height)).to_custom_err()?;
    render_in_section(&text_texture, x, y, section, canvas)

}





pub fn render_in_section (texture: &Texture, lx: i32, ly: i32, section: Rect, canvas: &mut WindowCanvas) -> Result<()> {
    let (mut width, mut height) = fns::get_texture_size(&texture);
    let (hx, hy) = (lx + width as i32, ly + height as i32);
    let (section_lx, section_ly) = (section.x(), section.y());
    let (section_width, section_height) = (section.width(), section.height());
    let (section_hx, section_hy) = (section_lx + section_width as i32, section_ly + section_height as i32);

    if
        (hx < 0) || (lx > section_width as i32) ||
        (hy < 0) || (ly > section_height as i32)
    {
        return Ok(());
    }

    let shown_lx = lx.max(0);
    let shown_ly = ly.max(0);
    let shown_hx = hx.min(section_width as i32);
    let shown_hy = hy.min(section_height as i32);
    let src_lx = shown_lx - lx;
    let src_ly = shown_ly - ly;
    let src_hx = shown_hx - hx + width as i32;
    let src_hy = shown_hy - hy + width as i32;

    let src = Rect::new(src_lx, src_ly, (src_hx - src_lx) as u32, (src_hy - src_ly) as u32);
    let dest = Rect::new(shown_lx + section_lx, shown_ly + section_ly, (shown_hx - shown_lx) as u32, (shown_hy - shown_ly) as u32);
    canvas.copy(texture, Some(src), dest).to_custom_err()

}



pub fn div (a: u32, b: f64) -> u32 {
    (a as f64 / b) as u32
}
