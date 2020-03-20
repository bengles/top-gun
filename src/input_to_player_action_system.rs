use specs::prelude::*;
use super::*;

pub struct InputToPlayerActionSystem;

impl<'a> System<'a> for InputToPlayerActionSystem {
    type SystemData = (
        WriteStorage<'a, PlayerActionMap>
    );

    fn run(&mut self, (player_action_maps): Self::SystemData) {
    }
}