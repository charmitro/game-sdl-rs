use specs::prelude::*;

use crate::components::*;

pub struct Scene;

impl<'a> System<'a> for Scene {
    type SystemData = WriteStorage<'a, SceneStatus>;

    fn run(&mut self, data: Self::SystemData) {}
}
