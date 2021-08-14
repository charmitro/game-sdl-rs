mod entities;

use entities::player::*;
use std::time::Duration;

use sdl2::event::Event;
use sdl2::image::{self, InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, WindowCanvas};

extern crate sdl2;

const PLAYER_MOVEMENT_SPEED: i32 = 2;

fn render(
    canvas: &mut WindowCanvas,
    color: Color,
    texture: &Texture,
    player: &Player,
) -> Result<(), String> {
    canvas.set_draw_color(color);
    canvas.clear();

    let (width, height) = canvas.output_size()?;

    let (frame_width, frame_height) = player.sprite.size();
    let current_frame = Rect::new(
        player.sprite.x() + frame_width as i32 * player.current_frame,
        player.sprite.y()
            + frame_height as i32 * Player::direction_spritesheet_row(player.direction),
        frame_width,
        frame_height,
    );

    let screen_pos = player.position + Point::new(width as i32 / 2, height as i32 / 2);
    let screen_rect = Rect::from_center(screen_pos, frame_width, frame_height);

    canvas.copy(texture, current_frame, screen_rect)?;

    canvas.present();

    Ok(())
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG);

    let window = video_subsystem
        .window("d", 800, 600)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture("assets/bardo.png").unwrap();

    let mut player = Player {
        name: String::from("dad"),
        health: 13,
        position: Point::new(0, 0),
        sprite: Rect::new(0, 0, 26, 26),
        speed: 5,
        direction: entities::player::Direction::Right,
        current_frame: 0,
    };

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(sdl2::keyboard::Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    repeat: false,
                    ..
                } => {
                    player.speed = PLAYER_MOVEMENT_SPEED;
                    player.direction = Direction::Left;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    repeat: false,
                    ..
                } => {
                    player.speed = PLAYER_MOVEMENT_SPEED;
                    player.direction = Direction::Right;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    repeat: false,
                    ..
                } => {
                    player.speed = PLAYER_MOVEMENT_SPEED;
                    player.direction = Direction::Up;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    repeat: false,
                    ..
                } => {
                    player.speed = PLAYER_MOVEMENT_SPEED;
                    player.direction = Direction::Down;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Left),
                    repeat: false,
                    ..
                }
                | Event::KeyUp {
                    keycode: Some(Keycode::Right),
                    repeat: false,
                    ..
                }
                | Event::KeyUp {
                    keycode: Some(Keycode::Up),
                    repeat: false,
                    ..
                }
                | Event::KeyUp {
                    keycode: Some(Keycode::Down),
                    repeat: false,
                    ..
                } => {
                    player.speed = 0;
                }
                _ => {}
            }
        }

        player.update_player();
        i = (i + 1) % 255;
        render(&mut canvas, Color::RGB(i, 64, 255 - i), &texture, &player)?;

        ::std::thread::sleep(Duration::new(0, 1_000_000u32 / 60));
    }

    Ok(())
}
