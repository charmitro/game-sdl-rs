use specs::prelude::*;

use crate::components::*;

use crate::utils::*;

const PLAYER_MOVEMENT_SPEED: i32 = 3;

pub struct Keyboard;

impl<'a> System<'a> for Keyboard {
    type SystemData = (
        ReadExpect<'a, Option<MovementCommand>>,
        ReadStorage<'a, KeyboardControlled>,
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, SceneStatus>,
    );

    fn run(&mut self, mut data: Self::SystemData) {
        let movement_command = match &*data.0 {
            Some(movement_command) => movement_command,
            None => return,
        };

        for (_, vel, _status) in (&data.1, &mut data.2, &mut data.3).join() {
            match movement_command {
                &MovementCommand::Move(direction) => {
                    vel.speed = PLAYER_MOVEMENT_SPEED;
                    vel.direction = direction;
                }
                MovementCommand::Stop => vel.speed = 0,
                MovementCommand::MoveStatus() => {
                    if _status.status == Status::Start {
                        _status.status = Status::Pause;
                    } else {
                        _status.status = Status::Start;
                    }
                }
            }
        }
    }
}
