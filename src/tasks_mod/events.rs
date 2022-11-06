use std::time::Instant;

use crate::{data_mod::{program_data::*, settings::*, errors::*, errors::Result::*}, fns};

use sdl2::{event::Event, keyboard::Keycode};



pub fn handle_event (event: Event, program_data: &ProgramData) -> Result<()> {
    match event {

        Event::Quit {..}  => {
            *program_data.exit.lock().unwrap() = true;
            return Ok(());
        }

        Event::KeyDown {keycode, repeat, .. } => {
            match keycode {
                Some(keycode) => handle_key_down(keycode, repeat, program_data),
                None => Ok(())
            }
        }

        Event::KeyUp {keycode, repeat, .. } => {
            match keycode {
                Some(keycode) => handle_key_up(keycode, repeat, program_data),
                None => Ok(())
            }
        }

        _ => Ok(())

    }
}





pub fn handle_key_down (keycode: Keycode, repeat: bool, program_data: &ProgramData) -> Result<()> {
    match keycode {

        Keycode::Escape => {
            handle_esc_pressed (program_data);
            Ok(())
        }

        Keycode::Up => {
            move_cursors(program_data, move_cursor_up)
        }
        Keycode::Down => {
            move_cursors(program_data, move_cursor_down)
        }
        Keycode::Left => {
            move_cursors(program_data, move_cursor_left)
        }
        Keycode::Right => {
            move_cursors(program_data, move_cursor_right)
        }

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

        _ => Ok(())

    }
}





pub fn handle_key_up (keycode: Keycode, repeat: bool, program_data: &ProgramData) -> Result<()> {
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





pub fn handle_esc_pressed (program_data: &ProgramData) {
    *program_data.exit.lock().unwrap() = true;
}



pub fn move_cursors (program_data: &ProgramData, move_fn: impl Fn(&mut File, usize)) -> Result<()> {
    let mut files = program_data.files.lock().unwrap();
    let mut current_file = match fns::get_current_file(program_data, &mut files)? {
        Some(v) => v,
        None => return Ok(()),
    };
    for i in 0..current_file.cursors.len() {
        move_fn(current_file, i);
    }
    *program_data.cursor_place_instant.lock().unwrap() = Instant::now();
    Ok(())
}



pub fn move_cursor_up (current_file: &mut File, cursor_num: usize) {
    let mut cursor = &mut current_file.cursors[cursor_num];
    cursor.y = cursor.y.max(1) - 1;
    let max_x = current_file.contents[cursor.y as usize].len() as u32;
    cursor.x = cursor.wanted_x.min(max_x);
}



pub fn move_cursor_down (current_file: &mut File, cursor_num: usize) {
    let mut cursor = &mut current_file.cursors[cursor_num];
    let max_y = (current_file.contents.len() - 1) as u32;
    cursor.y = cursor.y.min(max_y - 1) + 1;
    let max_x = current_file.contents[cursor.y as usize].len() as u32;
    cursor.x = cursor.wanted_x.min(max_x);
}



pub fn move_cursor_left (current_file: &mut File, cursor_num: usize) {
    let mut cursor = &mut current_file.cursors[cursor_num];
    'main: {
        if cursor.x > 0 {
            cursor.x -= 1;
        } else {
            if cursor.y == 0 {break 'main;}
            cursor.y -= 1;
            cursor.x = current_file.contents[cursor.y as usize].len() as u32;
        }
    }
    cursor.wanted_x = cursor.x;
}



pub fn move_cursor_right (current_file: &mut File, cursor_num: usize) {
    let mut cursor = &mut current_file.cursors[cursor_num];
    let max_x = current_file.contents[cursor.y as usize].len() as u32;
    'main: {
        if cursor.x < max_x {
            cursor.x += 1;
        } else {
            if cursor.y as usize == current_file.contents.len() - 1 {break 'main;}
            cursor.y += 1;
            cursor.x = 0;
        }
    }
    cursor.wanted_x = cursor.x;
}

