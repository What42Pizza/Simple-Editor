use crate::prelude::*;
use sdl2::{event::Event, keyboard::Keycode};



pub fn handle_event (event: Event, program_data: &ProgramData) -> Result<()> {
    let mut files = program_data.files.lock().unwrap();
    let current_file = match fns::get_current_file(program_data, &mut files)? {
        Some(v) => v,
        None => return Ok(()),
    };
    match event {

        Event::Quit {..}  => {
            *program_data.exit.lock().unwrap() = true;
            Ok(())
        }

        Event::KeyDown {keycode: Some(keycode), repeat, timestamp, ..} => handle_key_down(keycode, repeat, program_data, current_file, timestamp),
        Event::KeyUp {keycode: Some(keycode), repeat, ..} => handle_key_up(keycode, repeat, program_data, current_file),

        Event::TextInput {text, timestamp, ..} => handle_text_input(&text, program_data, current_file, timestamp),

        _ => Ok(())

    }
}





pub fn handle_key_down (keycode: Keycode, repeat: bool, program_data: &ProgramData, current_file: &mut File, timestamp: u32) -> Result<()> {
    if timestamp == *program_data.last_text_input_timestamp.lock().unwrap() {return Ok(());}
    match keycode {



        Keycode::Up    => run_fn_at_cursors(move_cursor_up_fn   , program_data, current_file),
        Keycode::Down  => run_fn_at_cursors(move_cursor_down_fn , program_data, current_file),
        Keycode::Left  => run_fn_at_cursors(move_cursor_left_fn , program_data, current_file),
        Keycode::Right => run_fn_at_cursors(move_cursor_right_fn, program_data, current_file),
        Keycode::End   => run_fn_at_cursors(move_cursor_end_fn  , program_data, current_file),

        Keycode::LShift | Keycode::RShift => {
            program_data.keys_pressed.lock().unwrap().shift_pressed = true;
            Ok(())
        }
        Keycode::LCtrl | Keycode::RCtrl => {
            program_data.keys_pressed.lock().unwrap().control_pressed = true;
            Ok(())
        }
        Keycode::LAlt | Keycode::RAlt => {
            program_data.keys_pressed.lock().unwrap().alt_pressed = true;
            Ok(())
        }

        Keycode::Escape => handle_esc_pressed (program_data),



        Keycode::Backspace => run_fn_at_cursors(backspace_fn, program_data, current_file),
        Keycode::Delete => run_fn_at_cursors(delete_fn, program_data, current_file),
        Keycode::Return => run_fn_at_cursors(return_fn, program_data, current_file),



        _ => {
            println!("warning: unknown keycode {keycode:?}");
            Ok(())
        }



    }
}





pub fn handle_key_up (keycode: Keycode, repeat: bool, program_data: &ProgramData, current_file: &mut File) -> Result<()> {
    match keycode {

        Keycode::LShift | Keycode::RShift => {
            program_data.keys_pressed.lock().unwrap().shift_pressed = false;
            Ok(())
        }
        Keycode::LCtrl | Keycode::RCtrl => {
            program_data.keys_pressed.lock().unwrap().control_pressed = false;
            Ok(())
        }
        Keycode::LAlt | Keycode::RAlt => {
            program_data.keys_pressed.lock().unwrap().alt_pressed = false;
            Ok(())
        }

        _ => Ok(())

    }
}





pub fn handle_esc_pressed (program_data: &ProgramData) -> Result<()> {
    *program_data.exit.lock().unwrap() = true;
    Ok(())
}





pub fn run_fn_at_cursors (edit_fn: impl Fn(&mut File, usize) -> Result<()>, program_data: &ProgramData, current_file: &mut File) -> Result<()> {
    for i in 0..current_file.cursors.len() {
        edit_fn(current_file, i)?
    }
    *program_data.cursor_place_instant.lock().unwrap() = Instant::now();
    Ok(())
}





pub fn move_cursor_up_fn (current_file: &mut File, cursor_num: usize) -> Result<()> {
    let mut cursor = &mut current_file.cursors[cursor_num];
    cursor.y = cursor.y.max(1) - 1;
    let max_x = current_file.contents[cursor.y].len();
    cursor.x = cursor.wanted_x.min(max_x);
    Ok(())
}



pub fn move_cursor_down_fn (current_file: &mut File, cursor_num: usize) -> Result<()> {
    let mut cursor = &mut current_file.cursors[cursor_num];
    let max_y = current_file.contents.len() - 1;
    cursor.y = cursor.y.min(max_y - 1) + 1;
    let max_x = current_file.contents[cursor.y].len();
    cursor.x = cursor.wanted_x.min(max_x);
    Ok(())
}



pub fn move_cursor_left_fn (current_file: &mut File, cursor_num: usize) -> Result<()> {
    let mut cursor = &mut current_file.cursors[cursor_num];
    'main: {
        if cursor.x > 0 {
            cursor.x -= 1;
        } else {
            if cursor.y == 0 {break 'main;}
            cursor.y -= 1;
            cursor.x = current_file.contents[cursor.y].len();
        }
    }
    cursor.wanted_x = cursor.x;
    Ok(())
}



pub fn move_cursor_right_fn (current_file: &mut File, cursor_num: usize) -> Result<()> {
    let mut cursor = &mut current_file.cursors[cursor_num];
    let max_x = current_file.contents[cursor.y].len();
    'main: {
        if cursor.x < max_x {
            cursor.x += 1;
        } else {
            if cursor.y == current_file.contents.len() - 1 {break 'main;}
            cursor.y += 1;
            cursor.x = 0;
        }
    }
    cursor.wanted_x = cursor.x;
    Ok(())
}



pub fn move_cursor_end_fn (current_file: &mut File, cursor_num: usize) -> Result<()> {
    let mut cursor = &mut current_file.cursors[cursor_num];
    let max_x = current_file.contents[cursor.y].len();
    cursor.x = max_x;
    cursor.wanted_x = cursor.x;
    Ok(())
}





pub fn backspace_fn (current_file: &mut File, cursor_num: usize) -> Result<()> {
    let mut cursor = &mut current_file.cursors[cursor_num];
    let contents = &mut current_file.contents;
    'main: {

        if cursor.x == 0 {
            if cursor.y == 0 {return Ok(());}
            let bottom_line = &mut contents.remove(cursor.y);
            cursor.y -= 1;
            cursor.x = contents[cursor.y].len();
            contents[cursor.y].append(bottom_line);
            break 'main;
        }

        let current_line = &mut contents[cursor.y];
        current_line.remove(cursor.x - 1);
        cursor.x -= 1;

    }
    cursor.wanted_x = cursor.x;
    Ok(())
}



pub fn delete_fn (current_file: &mut File, cursor_num: usize) -> Result<()> {
    let mut cursor = &mut current_file.cursors[cursor_num];
    let contents = &mut current_file.contents;
    let current_line = &mut contents[cursor.y];

    if cursor.x == current_line.len() {
        if cursor.y == contents.len() - 1 {return Ok(());}
        let next_line = &mut contents.remove(cursor.y + 1);
        contents[cursor.y].append(next_line);
        return Ok(());
    }

    current_line.remove(cursor.x);
    cursor.wanted_x = cursor.x;
    Ok(())
}



pub fn return_fn (current_file: &mut File, cursor_num: usize) -> Result<()> {
    let mut cursor = &mut current_file.cursors[cursor_num];
    let contents = &mut current_file.contents;
    let current_line = &mut contents[cursor.y];
    
    let new_line = current_line.split_off(cursor.x);
    contents.insert(cursor.y + 1, new_line);

    cursor.y += 1;
    cursor.x = 0;
    cursor.wanted_x = cursor.x;
    Ok(())
}





fn handle_text_input (text: &str, program_data: &ProgramData, current_file: &mut File, timestamp: u32) -> Result<()> {
    let place_text_fn = |file: &mut File, cursor_num| {
        let cursor: &mut Cursor = &mut file.cursors[cursor_num];
        let current_line = &mut file.contents[cursor.y];
        let text_chars = text.chars().collect::<Vec<char>>();
        fns::insert_all(&text_chars, current_line, cursor.x);
        cursor.x += 1;
        Ok(())
    };
    run_fn_at_cursors(place_text_fn, program_data, current_file)?;
    *program_data.last_text_input_timestamp.lock().unwrap() = timestamp;
    Ok(())
}
