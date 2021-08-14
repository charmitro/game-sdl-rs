use sdl2::rect::{Point, Rect};

pub struct Player {
    pub name: String,
    pub health: i32,
    pub position: Point,
    pub sprite: Rect,
    pub speed: i32,
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
        })
    }

    pub fn get_name(&self) -> &str {
        return self.name.as_str();
    }

    pub fn get_health(&self) -> i32 {
        return self.health;
    }
}
