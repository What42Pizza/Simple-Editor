use crate::prelude::*;
use sdl2::{event::Event, keyboard::Keycode};



pub fn handle_event (event: Event, program_data: &ProgramData) -> Result<(), ProgramError> {
    let mut files = program_data.files.write();
    let current_file = fns::get_current_file_mut(program_data, &mut files)?;
    match event {

        Event::Quit {..}  => {
            *program_data.exit.write() = true;
            Ok(())
        }

        Event::KeyDown {keycode: Some(keycode), repeat, timestamp, ..} => handle_key_down(keycode, repeat, program_data, current_file, timestamp),
        Event::KeyUp {keycode: Some(keycode), repeat, ..} => handle_key_up(keycode, repeat, program_data, current_file),

        Event::TextInput {text, timestamp, ..} => handle_text_input(&text, program_data, current_file, timestamp),

        _ => Ok(())

    }
}





pub fn handle_key_down (keycode: Keycode, _repeat: bool, program_data: &ProgramData, current_file: Option<&mut File>, timestamp: u32) -> Result<(), ProgramError> {
    if timestamp == *program_data.last_text_input_timestamp.read() {return Ok(());}
    match keycode {



        Keycode::Up    if current_file.is_some() => run_fn_at_cursors(move_cursor_up_fn   , program_data, current_file.unwrap()),
        Keycode::Down  if current_file.is_some() => run_fn_at_cursors(move_cursor_down_fn , program_data, current_file.unwrap()),
        Keycode::Left  if current_file.is_some() => run_fn_at_cursors(move_cursor_left_fn , program_data, current_file.unwrap()),
        Keycode::Right if current_file.is_some() => run_fn_at_cursors(move_cursor_right_fn, program_data, current_file.unwrap()),
        Keycode::End   if current_file.is_some() => run_fn_at_cursors(move_cursor_end_fn  , program_data, current_file.unwrap()),

        Keycode::LShift | Keycode::RShift => {
            program_data.keys_pressed.write().shift_pressed = true;
            Ok(())
        }
        Keycode::LCtrl | Keycode::RCtrl => {
            program_data.keys_pressed.write().control_pressed = true;
            Ok(())
        }
        Keycode::LAlt | Keycode::RAlt => {
            program_data.keys_pressed.write().alt_pressed = true;
            Ok(())
        }

        Keycode::Escape => handle_esc_pressed (program_data),



        Keycode::Backspace if current_file.is_some() => run_fn_at_cursors(backspace_fn, program_data, current_file.unwrap()),
        Keycode::Delete if current_file.is_some() => run_fn_at_cursors(delete_fn, program_data, current_file.unwrap()),
        Keycode::Return if current_file.is_some() => run_fn_at_cursors(return_fn, program_data, current_file.unwrap()),



        _ => {
            println!("warning: unknown keycode {keycode:?}");
            Ok(())
        }



    }
}





pub fn handle_key_up (keycode: Keycode, _repeat: bool, program_data: &ProgramData, current_file: Option<&mut File>) -> Result<(), ProgramError> {
    let Some(_current_file) = current_file else {return Ok(());};
    match keycode {

        Keycode::LShift | Keycode::RShift => {
            program_data.keys_pressed.write().shift_pressed = false;
            Ok(())
        }
        Keycode::LCtrl | Keycode::RCtrl => {
            program_data.keys_pressed.write().control_pressed = false;
            Ok(())
        }
        Keycode::LAlt | Keycode::RAlt => {
            program_data.keys_pressed.write().alt_pressed = false;
            Ok(())
        }

        _ => Ok(())

    }
}





pub fn handle_esc_pressed (program_data: &ProgramData) -> Result<(), ProgramError> {
    *program_data.exit.write() = true;
    Ok(())
}





pub fn run_fn_at_cursors (cursor_fn: impl Fn(&mut File, usize, &ProgramData) -> Result<(), ProgramError>, program_data: &ProgramData, current_file: &mut File) -> Result<(), ProgramError> {
    for i in 0..current_file.cursors.len() {
        cursor_fn(current_file, i, program_data)?
    }
    remove_cursor_duplicates(&mut current_file.cursors);
    *program_data.cursor_place_instant.write() = Instant::now();
    Ok(())
}



pub fn remove_cursor_duplicates (cursors: &mut Vec<Cursor>) {
    // this is O(n^2), but it should be fine
    let mut cursors_to_remove = vec!();
    for (i, cursor_1) in cursors.iter().enumerate() {
        'inner: for cursor_2 in cursors.iter().skip(i + 1) {
            if cursor_1.x == cursor_2.x && cursor_1.y == cursor_2.y {
                cursors_to_remove.push(i);
                break 'inner;
            }
        }
    }
    for cursor_index in cursors_to_remove.iter().rev() {
        cursors.remove(*cursor_index);
    }
}





pub fn move_cursor_up_fn (current_file: &mut File, cursor_num: usize, program_data: &ProgramData) -> Result<(), ProgramError> {
    let mut cursor = &mut current_file.cursors[cursor_num];
    handle_cursor_selection_on_move(cursor, program_data);
    cursor.y = cursor.y.max(1) - 1;
    let max_x = current_file.contents[cursor.y].len();
    cursor.x = cursor.wanted_x.min(max_x);
    Ok(())
}



pub fn move_cursor_down_fn (current_file: &mut File, cursor_num: usize, program_data: &ProgramData) -> Result<(), ProgramError> {
    let mut cursor = &mut current_file.cursors[cursor_num];
    handle_cursor_selection_on_move(cursor, program_data);
    let max_y = current_file.contents.len() as isize - 1;
    cursor.y = ((cursor.y as isize).min(max_y - 1) + 1) as usize;
    let max_x = current_file.contents[cursor.y].len();
    cursor.x = cursor.wanted_x.min(max_x);
    Ok(())
}



pub fn move_cursor_left_fn (current_file: &mut File, cursor_num: usize, program_data: &ProgramData) -> Result<(), ProgramError> {
    let mut cursor = &mut current_file.cursors[cursor_num];
    handle_cursor_selection_on_move(cursor, program_data);
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



pub fn move_cursor_right_fn (current_file: &mut File, cursor_num: usize, program_data: &ProgramData) -> Result<(), ProgramError> {
    let mut cursor = &mut current_file.cursors[cursor_num];
    handle_cursor_selection_on_move(cursor, program_data);
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



pub fn move_cursor_end_fn (current_file: &mut File, cursor_num: usize, program_data: &ProgramData) -> Result<(), ProgramError> {
    let mut cursor = &mut current_file.cursors[cursor_num];
    handle_cursor_selection_on_move(cursor, program_data);
    let max_x = current_file.contents[cursor.y].len();
    cursor.x = max_x;
    cursor.wanted_x = cursor.x;
    Ok(())
}



pub fn handle_cursor_selection_on_move (cursor: &mut Cursor, program_data: &ProgramData) {
    if program_data.keys_pressed.read().shift_pressed {
        if cursor.selection_start.is_none() {
            cursor.selection_start = Some((cursor.x, cursor.y));
        }
    } else {
        cursor.selection_start = None;
    }
}





pub fn backspace_fn (current_file: &mut File, cursor_num: usize, _program_data: &ProgramData) -> Result<(), ProgramError> {
    let mut cursor = &mut current_file.cursors[cursor_num];
    let contents = &mut current_file.contents;
    'main: {

        if cursor.selection_start.is_some() {
            delete_selected_area(contents, cursor);
            break 'main;
        }

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



pub fn delete_fn (current_file: &mut File, cursor_num: usize, _program_data: &ProgramData) -> Result<(), ProgramError> {
    let mut cursor = &mut current_file.cursors[cursor_num];
    let contents = &mut current_file.contents;
    let current_line = &mut contents[cursor.y];

    if cursor.selection_start.is_some() {
        delete_selected_area(contents, cursor);
        cursor.wanted_x = cursor.x;
        return Ok(());
    }

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



pub fn return_fn (current_file: &mut File, cursor_num: usize, _program_data: &ProgramData) -> Result<(), ProgramError> {
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



pub fn delete_selected_area (contents: &mut [Vec<char>], cursor: &Cursor) {
    let (start_x, start_y) = cursor.selection_start.unwrap();
    let (end_x, end_y) = (cursor.x, cursor.y);
    println!("WIP: delete area between ({start_x}, {start_y}) and ({end_x}, {end_y})");
}





fn handle_text_input (text: &str, program_data: &ProgramData, current_file: Option<&mut File>, timestamp: u32) -> Result<(), ProgramError> {
    let Some(current_file) = current_file else {return Ok(());};
    let place_text_fn = |file: &mut File, cursor_num: usize, _program_data: &ProgramData| {
        let cursor: &mut Cursor = &mut file.cursors[cursor_num];
        let current_line = &mut file.contents[cursor.y];
        let text_chars = text.chars().collect::<Vec<char>>();
        fns::insert_all(&text_chars, current_line, cursor.x);
        cursor.x += 1;
        Ok(())
    };
    run_fn_at_cursors(place_text_fn, program_data, current_file)?;
    *program_data.last_text_input_timestamp.write() = timestamp;
    Ok(())
}
