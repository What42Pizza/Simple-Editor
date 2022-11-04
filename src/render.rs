use crate::{data::{program_data::*, settings::*, errors::*, errors::Result::*}, fns};

use std::{sync::MutexGuard, thread, time::Duration};
use sdl2::{video::WindowContext, ttf::Font, pixels::Color,
    render::{WindowCanvas, TextureCreator, Texture},
    rect::{Rect, Point}
};



pub fn render(canvas: &mut WindowCanvas, program_data: &ProgramData, textures: &mut ProgramTextures<'_>, texture_creator: &TextureCreator<WindowContext>, font: &Font) -> Result<()> {

    // pause tasks
    *program_data.tasks_paused.lock().unwrap() = true;
    while *program_data.tasks_ongoing.lock().unwrap() {
        thread::sleep(Duration::from_millis(1));
    }

    // render (and resume tasks)
    prepare_canvas(canvas, program_data, textures, texture_creator, font)?;
    *program_data.tasks_paused.lock().unwrap() = false;
    canvas.present();
    Ok(())

}





pub fn prepare_canvas (canvas: &mut WindowCanvas, program_data: &ProgramData, textures: &mut ProgramTextures<'_>, texture_creator: &TextureCreator<WindowContext>, font: &Font) -> Result<()> {

    // get data
    let settings_mutex = program_data.settings.lock().unwrap();
    let settings = settings_mutex.as_ref().expect("Error: settings is none");
    let (width, height) = canvas.output_size().to_custom_err()?;
    let buttons_bottom_y = div(height, 20.);

    // clear
    canvas.set_draw_color(settings.background_color);
    canvas.clear();


    // render top buttons
    canvas.set_draw_color(fns::blend_colors(settings.background_color, Color::RGB(0, 0, 0), 0.5));
    canvas.draw_line(Point::new(0, buttons_bottom_y as i32), Point::new(width as i32, buttons_bottom_y as i32)).to_custom_err()?;


    // render text
    let files = program_data.files.lock().unwrap();
    let current_file = match get_file_to_render(program_data, &files) {
        Some(v) => v,
        None => {return Ok(());}
    };

    let text_section = Rect::new(0, buttons_bottom_y as i32, width, height - buttons_bottom_y);
    let text_spacing = (settings.font_size as f64 * settings.font_spacing) as u32;
    let padding = div(width, 80.) as i32;
    for (i, current_line) in current_file.contents.iter().enumerate() {
        render_text(current_line, padding, i as i32 * text_spacing as i32 + padding, &text_section, font, canvas, texture_creator, textures, settings)?;
    }


    // render cursors
    let cursor_place_instant = program_data.cursor_place_instant.lock().unwrap();
    let time_since_cursor_place = cursor_place_instant.elapsed().as_secs_f64();
    let cursor_flashing_speed = settings.cursor_flashing_speed;
    if time_since_cursor_place % cursor_flashing_speed < cursor_flashing_speed / 2. {
        let cursor_width = (width as f64 * settings.cursor_width) as u32;
        let cursor_height = (settings.font_size as f64 * settings.cursor_height) as u32;
        let cursor_color = settings.cursor_color;
        for cursor in &current_file.cursors {
            render_cursor(cursor, canvas, &text_section, cursor_width, cursor_height, cursor_color, text_spacing);
        }
    }

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



pub fn render_text (text: &[char], x: i32, y: i32, section: &Rect, font: &Font, canvas: &mut WindowCanvas, texture_creator: &TextureCreator<WindowContext>, textures: &ProgramTextures, settings: &ProgramSettings) -> Result<()> {
    let text_width = settings.font_size as i32 * 3 / 4;
    for (i, char) in text.iter().enumerate() {
        let char = *char as usize;
        if char < 256 {
            let char_texture = &textures.ascii_chars[char];
            render_in_section(char_texture, x + i as i32 * text_width, y, section, canvas)?;
        } else {
            return err("WIPChar", &("cannot render char ".to_string() + &char.to_string()));
        }
    }
    Ok(())
}



pub fn render_cursor (cursor: &Cursor, canvas: &mut WindowCanvas, text_section: &Rect, cursor_wdith: u32, cursor_height: u32, cursor_color: Color, text_spacing: u32) -> Result<()> {

    //canvas.draw_rect()
    Ok(())

}



pub fn get_cursor_screen_position (cursor: &Cursor, text_section: &Rect) -> (u32, u32) {

    (0, 0)
}





pub fn render_in_section (texture: &Texture, lx: i32, ly: i32, section: &Rect, canvas: &mut WindowCanvas) -> Result<()> {
    let (width, height) = fns::get_texture_size(texture);
    let (hx, hy) = (lx + width as i32, ly + height as i32);
    let (section_lx, section_ly) = (section.x(), section.y());
    let (section_width, section_height) = (section.width(), section.height());

    /*
    // probably not needed?
    if
        (hx < 0) || (lx > section_width as i32) ||
        (hy < 0) || (ly > section_height as i32)
    {
        return Ok(());
    }
    */

    let shown_lx = lx.max(0);
    let shown_ly = ly.max(0);
    let shown_hx = hx.min(section_width as i32);
    let shown_hy = hy.min(section_height as i32);
    let src_lx = shown_lx - lx;
    let src_ly = shown_ly - ly;
    let src_hx = shown_hx - hx + width as i32;
    let src_hy = shown_hy - hy + height as i32;

    let src = Rect::new(src_lx, src_ly, (src_hx - src_lx) as u32, (src_hy - src_ly) as u32);
    let dest = Rect::new(shown_lx + section_lx, shown_ly + section_ly, (shown_hx - shown_lx) as u32, (shown_hy - shown_ly) as u32);
    canvas.copy(texture, Some(src), dest).to_custom_err()

}



pub fn div (a: u32, b: f64) -> u32 {
    (a as f64 / b) as u32
}
