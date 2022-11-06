use crate::{data_mod::{program_data::*, settings::*, errors::*, errors::Result::*}, fns};

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
    let mut files = program_data.files.lock().unwrap();
    let current_file = match fns::get_current_file(program_data, &mut files)? {
        Some(v) => v,
        None => return Ok(()),
    };

    let text_section = Rect::new(0, buttons_bottom_y as i32, width, height - buttons_bottom_y);
    //let text_spacing = (settings.font_size as f64 * settings.font_spacing) as u32;
    for (i, current_line) in current_file.contents.iter().enumerate() {
        render_file_text(current_line, i as u32, &text_section, font, canvas, texture_creator, textures, settings)?;
    }


    // render cursors
    let cursor_place_instant = program_data.cursor_place_instant.lock().unwrap();
    let time_since_cursor_place = cursor_place_instant.elapsed().as_secs_f64();
    let cursor_flashing_speed = settings.cursor_flashing_speed;
    if time_since_cursor_place % cursor_flashing_speed < cursor_flashing_speed / 2. {
        let cursor_width = (width as f64 * settings.cursor_width) as u32;
        let cursor_height = (settings.font_size as f64 * settings.cursor_height) as u32;
        canvas.set_draw_color(settings.cursor_color);
        for cursor in &current_file.cursors {
            render_cursor(cursor, canvas, &text_section, cursor_width, cursor_height, settings)?;
        }
    }

    Ok(())

}



pub fn render_file_text (text: &[char], char_y: u32, section: &Rect, font: &Font, canvas: &mut WindowCanvas, texture_creator: &TextureCreator<WindowContext>, textures: &ProgramTextures, settings: &ProgramSettings) -> Result<()> {
    let text_width = settings.font_size as i32 * 3 / 4;
    for (i, char) in text.iter().enumerate() {
        let char = *char as usize;
        if char < 256 {
            let char_texture = &textures.ascii_chars[char];
            let (x, y) = get_char_position(i as u32, char_y, section, settings);
            let (width, height) = fns::get_texture_size(char_texture);
            let (src, dest) = clamp_to_section(&Rect::new(x, y, width, height), section);
            canvas.copy(char_texture, Some(src), dest).to_custom_err()?;
        } else {
            return err("WIPChar", &("cannot render char ".to_string() + &char.to_string()));
        }
    }
    Ok(())
}



pub fn render_cursor (cursor: &Cursor, canvas: &mut WindowCanvas, section: &Rect, cursor_width: u32, cursor_height: u32, settings: &ProgramSettings) -> Result<()> {
    let (cursor_x, cursor_y) = get_char_position(cursor.x, cursor.y, section, settings);
    let y_offset = (settings.font_size * 3 / 32) as i32;
    let cursor_rect = Rect::new(cursor_x, cursor_y + y_offset, cursor_width, cursor_height);
    canvas.draw_rect(clamp_to_section(&cursor_rect, section).1).to_custom_err()
}





pub fn get_char_position (char_x: u32, char_y: u32, section: &Rect, settings: &ProgramSettings) -> (i32, i32) {
    let padding = div(section.width(), 80.) as i32;
    let char_height = settings.font_size as u32;
    let char_width = char_height * 3 / 4;
    let char_spacing = (char_height as f64 * settings.font_spacing) as u32;
    ((char_x * char_width) as i32 + padding, (char_y * char_spacing) as i32 + padding)
}





pub fn clamp_to_section (rect: &Rect, section: &Rect) -> (Rect, Rect) {
    let (lx, ly) = (rect.x, rect.y);
    let (width, height) = (rect.width(), rect.height());
    let (hx, hy) = (lx + width as i32, ly + height as i32);
    let (section_lx, section_ly) = (section.x(), section.y());
    let (section_width, section_height) = (section.width(), section.height());

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
    (src, dest)

}



pub fn div (a: u32, b: f64) -> u32 {
    (a as f64 / b) as u32
}
