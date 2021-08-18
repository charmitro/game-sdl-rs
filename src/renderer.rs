use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, WindowCanvas};
use specs::prelude::*;

use crate::components::*;

pub type SystemData<'a> = (
    ReadStorage<'a, Position>,
    ReadStorage<'a, Sprite>,
    ReadStorage<'a, SceneStatus>,
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

    for (pos, sprite, scene_status) in (&data.0, &data.1, &data.2).join() {
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
            canvas.set_draw_color(Color::MAGENTA);
            canvas.clear();
            canvas.copy(
                &textures[1],
                None,
                Rect::new(80, 200, 200, 200),
            )?;
        }
    }

    canvas.present();

    Ok(())
}
