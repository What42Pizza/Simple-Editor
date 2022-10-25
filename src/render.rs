use crate::{data::{program_data::*, settings::*, errors::*, errors::Result::*}, fns};

use sdl2::{render::WindowCanvas};



pub fn render(canvas: &mut WindowCanvas, program_data: &ProgramData, textures: &ProgramTextures<'_>) -> Result<()> {
    let (width, height) = canvas.output_size().to_custom_err()?;

    //canvas.set_draw_color(Color::RGB(255, 0, 255));
    //canvas.clear();

    // finish
    canvas.present();
    Ok(())

}
