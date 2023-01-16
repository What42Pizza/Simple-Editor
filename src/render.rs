use crate::prelude::*;
use sdl2::{video::WindowContext, ttf::Font, pixels::Color,
    render::{WindowCanvas, TextureCreator, BlendMode},
    rect::{Rect, Point}
};



pub fn render(canvas: &mut WindowCanvas, program_data: &ProgramData, textures: &mut ProgramTextures<'_>, texture_creator: &TextureCreator<WindowContext>, font: &Font) -> Result<()> {

    // render (and resume tasks)
    prepare_canvas(canvas, program_data, textures, texture_creator, font)?;

    'frame_timing: {
        let settings_mutex = program_data.settings.borrow();
        if let FrameTimingSetting::Maxxed(mut max_frame_time) = settings_mutex.as_ref().unwrap().frame_timing {
            max_frame_time *= 1000;
            let elapsed_time = program_data.last_frame_instant.borrow().elapsed().as_micros() as usize;
            if elapsed_time > max_frame_time {break 'frame_timing;}
            let time_to_sleep = max_frame_time - elapsed_time;
            spin_sleep::sleep(Duration::from_micros(time_to_sleep as u64));
        }
    }
    *program_data.last_frame_instant.borrow_mut() = Instant::now();

    canvas.present();
    Ok(())

}





pub fn prepare_canvas (canvas: &mut WindowCanvas, program_data: &ProgramData, textures: &mut ProgramTextures<'_>, texture_creator: &TextureCreator<WindowContext>, font: &Font) -> Result<()> {

    // get data
    let settings_mutex = program_data.settings.borrow();
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
    let mut files = program_data.files.borrow_mut();
    let current_file = match fns::get_current_file(program_data, &files)? {
        Some(v) => v,
        None => return Ok(()),
    };

    let text_section = Rect::new(0, buttons_bottom_y as i32, width, height - buttons_bottom_y);
    let text_spacing = (settings.font_size as f64 * settings.font_spacing) as u32;
    for (i, current_line) in current_file.contents.iter().enumerate() {
        render_text_line(current_line, i, &text_section, font, canvas, texture_creator, textures, settings)?;
    }


    // render cursors
    let cursor_place_instant = program_data.cursor_place_instant.borrow();
    let time_since_cursor_place = cursor_place_instant.elapsed().as_secs_f64();
    let cursor_flashing_speed = settings.cursor_flashing_speed;
    let render_cursor_lines = time_since_cursor_place % cursor_flashing_speed < cursor_flashing_speed / 2.;
    let cursor_width = (width as f64 * settings.cursor_width) as u32;
    let cursor_height = (settings.font_size as f64 * settings.cursor_height) as u32;
    for cursor in &current_file.cursors {
        render_cursor(cursor, cursor_width, cursor_height, render_cursor_lines, current_file, canvas, &text_section, settings)?;
    }

    Ok(())

}



pub fn render_text_line (text: &[char], text_y: usize, section: &Rect, font: &Font, canvas: &mut WindowCanvas, texture_creator: &TextureCreator<WindowContext>, textures: &ProgramTextures, settings: &ProgramSettings) -> Result<()> {
    for (i, char) in text.iter().enumerate() {
        let char = *char as usize;
        if char < 256 {
            let char_texture = &textures.ascii_chars[char];
            let (x, y) = get_char_position(i, text_y, section, settings);
            let (width, height) = fns::get_texture_size(char_texture);
            let (src, dest) = clamp_to_section(&Rect::new(x, y, width, height), section);
            canvas.copy(char_texture, Some(src), dest).to_custom_err()?;
        } else {
            return err("WIPChar", &("cannot render char ".to_string() + &char.to_string()));
        }
    }
    Ok(())
}



pub fn render_cursor (cursor: &Cursor, cursor_width: u32, cursor_height: u32, render_cursor_lines: bool, current_file: &File, canvas: &mut WindowCanvas, section: &Rect, settings: &ProgramSettings) -> Result<()> {

    // render selection
    if let Some((mut selection_start_x, mut selection_start_y)) = cursor.selection_start {
        let (mut selection_end_x, mut selection_end_y) = (cursor.x, cursor.y);
        canvas.set_blend_mode(BlendMode::Blend);
        canvas.set_draw_color(settings.cursor_selection_color);
        if selection_start_y == cursor.y {
            if selection_start_x > selection_end_x {(selection_start_x, selection_end_x) = (selection_end_x, selection_start_x);}
            render_rect_over_chars(selection_start_x, selection_end_x, selection_end_y, cursor_height, canvas, section, settings)?;
        } else {
            if selection_start_y > selection_end_y {(selection_start_x, selection_start_y, selection_end_x, selection_end_y) = (selection_end_x, selection_end_y, selection_start_x, selection_start_y);}
            let contents = &current_file.contents;
            render_rect_over_chars(selection_start_x, contents[selection_start_y].len() + 1, selection_start_y, cursor_height, canvas, section, settings)?;
            for (i, current_line) in contents.iter().enumerate().take(selection_end_y).skip(selection_start_y + 1) {
                render_rect_over_chars(0, current_line.len() + 1, i, cursor_height, canvas, section, settings)?;
            }
            render_rect_over_chars(0, selection_end_x, selection_end_y, cursor_height, canvas, section, settings)?;
        }
        canvas.set_blend_mode(BlendMode::None);
    }

    if !render_cursor_lines {return Ok(());}

    // render cursor line
    let y_offset = (settings.font_size * 3 / 32) as i32;
    canvas.set_draw_color(settings.cursor_color);
    let (cursor_x, cursor_y) = get_char_position(cursor.x, cursor.y, section, settings);
    let cursor_rect = Rect::new(cursor_x, cursor_y + y_offset, cursor_width, cursor_height);
    canvas.fill_rect(clamp_to_section(&cursor_rect, section).1).to_custom_err()

}



pub fn render_rect_over_chars (x_pos_1: usize, x_pos_2: usize, y_pos: usize, char_height: u32, canvas: &mut WindowCanvas, section: &Rect, settings: &ProgramSettings) -> Result<()> {
    let y_offset = (settings.font_size * 3 / 32) as i32;
    let x_offset = -((settings.font_size * 1 / 32) as i32);
    let (mut char_x_1, char_y) = get_char_position(x_pos_1, y_pos, section, settings);
    let (mut char_x_2, char_y) = get_char_position(x_pos_2, y_pos, section, settings);
    let selection_rect = Rect::new(char_x_1 + x_offset, char_y + y_offset, (char_x_2 - char_x_1) as u32, char_height);
    canvas.fill_rect(clamp_to_section(&selection_rect, section).1).to_custom_err()
}





pub fn get_char_position (char_x: usize, char_y: usize, section: &Rect, settings: &ProgramSettings) -> (i32, i32) {
    let padding = div(section.width(), 80.) as i32;
    let char_height = settings.font_size;
    let char_width = char_height * 11 / 16;
    let char_spacing = (char_height as f64 * settings.font_spacing) as u32;
    ((char_x as u32 * char_width) as i32 + padding, (char_y as u32 * char_spacing) as i32 + padding)
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
