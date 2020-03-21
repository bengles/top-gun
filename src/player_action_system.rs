use super::*;
use specs::prelude::*;

pub struct PlayerActionSystem;

impl PlayerActionSystem {
    pub const MOVE_SPEED: f32 = 5.0;
}

impl<'a> System<'a> for PlayerActionSystem {
    type SystemData = (
        ReadStorage<'a, PlayerActionMap>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, RigidBody>,
    );

    fn run(&mut self, (action_maps, mut transforms, mut rigidbodies): Self::SystemData) {
        for (action_map, transform, rigidbody) in (&action_maps, &mut transforms, &mut rigidbodies).join() {
            rigidbody.velocity = action_map.desired_move_direction * PlayerActionSystem::MOVE_SPEED;
            let heading = action_map.desired_heading_direction;
            transform.rotation = arctan2(heading.y, heading.x) + PI * 0.5;
        }
    }
}
