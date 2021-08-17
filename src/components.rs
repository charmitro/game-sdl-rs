use sdl2::rect::{Point, Rect};
use specs::prelude::*;
use specs_derive::Component;

use crate::utils::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Start,
    Pause,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct SceneStatus {
    pub status: Status,
}

#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct KeyboardControlled;

/// The current position of a given entity
#[derive(Component, Debug, Clone, Copy)]
#[storage(VecStorage)]
pub struct Position(pub Point);

/// The current speed and direction of a given entity
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Velocity {
    pub speed: i32,
    pub direction: Direction,
}

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct Sprite {
    /// The specific spritesheet to render from
    pub spritesheet: usize,
    /// The current region of the spritesheet to be rendered
    pub region: Rect,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct MovementAnimation {
    // The current frame in the animation of the direction this entity is moving in
    pub current_frame: usize,
    pub up_frames: Vec<Sprite>,
    pub down_frames: Vec<Sprite>,
    pub left_frames: Vec<Sprite>,
    pub right_frames: Vec<Sprite>,
}

impl MovementAnimation {
    pub fn init(player_spritesheet: usize, player_top_left_frame: Rect) -> MovementAnimation {
        return MovementAnimation {
            current_frame: 0,

            up_frames: character_animation_frames(
                player_spritesheet,
                player_top_left_frame,
                Direction::Up,
            ),
            down_frames: character_animation_frames(
                player_spritesheet,
                player_top_left_frame,
                Direction::Down,
            ),
            left_frames: character_animation_frames(
                player_spritesheet,
                player_top_left_frame,
                Direction::Left,
            ),
            right_frames: character_animation_frames(
                player_spritesheet,
                player_top_left_frame,
                Direction::Right,
            ),
        };
    }
}
