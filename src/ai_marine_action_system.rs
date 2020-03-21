use super::*;
use specs::*;

pub struct AiMarineActionSystem;

impl<'a> System<'a> for AiMarineActionSystem {
    type SystemData = (
        ReadStorage<'a, AI>,
        ReadStorage<'a, Transform>,
        WriteStorage<'a, MarineActionMap>,
    );

    fn run(&mut self, (ais, transforms, mut action_maps): Self::SystemData) {
        for (_ai, _transform, action_map) in (&ais, &transforms, &mut action_maps).join() {
            action_map.shoot = true;
            action_map.desired_heading_direction =
                utils::rotate_vector(action_map.desired_heading_direction, 1.0 * TO_RADIANS);
        }
    }
}
