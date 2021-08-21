use specs::prelude::*;

use crate::components::*;

pub struct Scene;

impl<'a> System<'a> for Scene {
    type SystemData = (WriteStorage<'a, SceneStatus>, ReadStorage<'a, Text>);

    fn run(&mut self, _data: Self::SystemData) {

    }
}
