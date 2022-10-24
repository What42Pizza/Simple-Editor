use crate::{data::{program_data::*, errors::*, errors::Result::*}, fns};

use std::sync::{Arc, Mutex};
use sdl2::{render::WindowCanvas};



pub fn render(canvas: &mut WindowCanvas, program_data: &Arc<Mutex<&mut ProgramData>>) -> Result<()> {
    let mut program_data = program_data.lock().unwrap();
    let textures = &program_data.textures;
    let (width, height) = canvas.output_size().to_custom_err()?;

    //canvas.set_draw_color(Color::RGB(255, 0, 255));
    //canvas.clear();

    // finish
    canvas.present();
    Ok(())

}
