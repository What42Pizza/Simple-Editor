use std::sync::MutexGuard;

use crate::{data::{program_data::*, settings::*, errors::*, errors::Result::*}, fns};

use sdl2::{render::WindowCanvas};



pub fn render(canvas: &mut WindowCanvas, program_data: &ProgramData, textures: &ProgramTextures<'_>) -> Result<()> {

    // get data
    let settings_mutex = program_data.settings.lock().unwrap();
    let settings = settings_mutex.as_ref().expect("Error: settings is none");
    let (width, height) = canvas.output_size().to_custom_err()?;

    // clear
    canvas.set_draw_color(settings.background_color);
    canvas.clear();

    // render
    let files = program_data.files.lock().unwrap();
    let file_contents = get_content_to_render(program_data, &files);

    // finish
    canvas.present();
    Ok(())

}





pub fn get_content_to_render<'a> (program_data: &ProgramData, files: &'a MutexGuard<Vec<File>>) -> Option<&'a Vec<String>> {

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

    Some(&files[current_file].contents)

}
