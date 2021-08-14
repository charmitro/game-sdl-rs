use sdl2::rect::{Point, Rect};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub health: i32,
    pub position: Point,
    pub sprite: Rect,
    pub speed: i32,
    pub direction: Direction,
    pub current_frame: i32,
}

impl Player {
    pub fn new(
        _name: &str,
        _health: i32,
        _position: Point,
        _rect: Rect,
        _speed: i32,
    ) -> Result<Player, String> {
        Ok(Player {
            name: String::from(_name),
            health: _health,
            position: _position,
            sprite: _rect,
            speed: _speed,
            direction: Direction::Down,
            current_frame: 0,
        })
    }

    pub fn get_name(&self) -> &str {
        return self.name.as_str();
    }

    pub fn get_health(&self) -> i32 {
        return self.health;
    }

    pub fn update_player(&mut self) {
        use self::Direction::*;

        match self.direction {
            Left => {
                self.position = self.position.offset(-self.speed, 0);
            }
            Right => {
                self.position = self.position.offset(self.speed, 0);
            }
            Up => {
                self.position = self.position.offset(0, -self.speed);
            }
            Down => {
                self.position = self.position.offset(0, self.speed);
            }
        }

        if self.speed != 0 {
            self.current_frame = (self.current_frame + 1) % 3;
        }
    }

    pub fn direction_spritesheet_row(direction: Direction) -> i32 {
        use self::Direction::*;
        match direction {
            Up => 3,
            Down => 0,
            Left => 1,
            Right => 2,
        }
    }
}
