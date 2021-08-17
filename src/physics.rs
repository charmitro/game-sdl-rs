use specs::prelude::*;

use crate::components::*;

pub struct Physics;

// TODO find a way to get window size dynamically
impl Physics {
    fn check_window_collision(pos: Position, direction: Direction) -> bool {
        use self::Direction::*;
        match direction {
            Left => {
                if pos.0.x() < ((-400) + 20) {
                    return false;
                } else {
                    return true;
                }
            }
            Right => {
                if pos.0.x() < ((400) - 20) {
                    return true;
                } else {
                    return false;
                }
            }
            Up => {
                if pos.0.y() > ((-300) + 20) {
                    return true;
                } else {
                    return false;
                }
            }
            Down => {
                if pos.0.y() < ((300) - 20) {
                    return true;
                } else {
                    return false;
                }
            }
        }
    }
}

impl<'a> System<'a> for Physics {
    type SystemData = (WriteStorage<'a, Position>, ReadStorage<'a, Velocity>);

    fn run(&mut self, mut data: Self::SystemData) {
        use self::Direction::*;

        for (pos, vel) in (&mut data.0, &data.1).join() {
            match vel.direction {
                Left => {
                    if Self::check_window_collision(pos.clone(), vel.direction) {
                        pos.0 = pos.0.offset(-vel.speed, 0);
                    }
                }
                Right => {
                    if Self::check_window_collision(pos.clone(), vel.direction) {
                        pos.0 = pos.0.offset(vel.speed, 0);
                    }
                }
                Up => {
                    if Self::check_window_collision(pos.clone(), vel.direction) {
                        pos.0 = pos.0.offset(0, -vel.speed);
                    }
                }
                Down => {
                    if Self::check_window_collision(pos.clone(), vel.direction) {
                        pos.0 = pos.0.offset(0, vel.speed);
                    }
                }
            }
        }
    }
}
