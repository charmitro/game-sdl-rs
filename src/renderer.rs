use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, WindowCanvas};
use specs::prelude::*;

use crate::components::*;

pub type SystemData<'a> = (
    ReadStorage<'a, Position>,
    ReadStorage<'a, Sprite>,
    ReadStorage<'a, SceneStatus>,
    ReadStorage<'a, Text>,
);

pub fn render(
    canvas: &mut WindowCanvas,
    background: Color,
    textures: &[Texture],
    data: SystemData,
) -> Result<(), String> {
    canvas.set_draw_color(background);
    canvas.clear();

    let (width, height) = canvas.output_size()?;

    for (pos, sprite, scene_status, text) in (&data.0, &data.1, &data.2, &data.3).join() {
        if scene_status.status == Status::Start {
            let current_frame = sprite.region;

            let screen_position = pos.0 + Point::new(width as i32 / 2, height as i32 / 2);
            let screen_rect = Rect::from_center(
                screen_position,
                current_frame.width(),
                current_frame.height(),
            );

            canvas.copy(&textures[sprite.spritesheet], current_frame, screen_rect)?;
        } else if scene_status.status == Status::Pause {

            let current_frame = text.region;

            let screen_position = pos.0 + Point::new(width as i32 / 2, height as i32 / 2);
            let screen_rect = Rect::from_center(
                screen_position,
                current_frame.width(),
                current_frame.height(),
            );


            canvas.copy(&textures[1], current_frame, screen_rect)?;
        }
    }

    canvas.present();

    Ok(())
}
