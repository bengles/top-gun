use super::*;
use specs::prelude::*;

pub struct InputToMarineActionSystem;

impl<'a> System<'a> for InputToMarineActionSystem {
    type SystemData = (
        ReadStorage<'a, Player>,
        ReadStorage<'a, Transform>,
        WriteStorage<'a, MarineActionMap>,
        ReadExpect<'a, Input>,
    );

    fn run(&mut self, (players, transforms, mut action_maps, input): Self::SystemData) {
        for (_player, transform, player_action_map) in
            (&players, &transforms, &mut action_maps).join()
        {
            let mut move_direction: Vector2 = Vector2::zeros();
            player_action_map.shoot = input.keys_pressed[&Key::Mouse1];
            if input.keys_pressed[&Key::W] {
                move_direction.y = 1.0;
            }
            if input.keys_pressed[&Key::S] {
                move_direction.y = -1.0;
            }
            if input.keys_pressed[&Key::D] {
                move_direction.x = 1.0;
            }
            if input.keys_pressed[&Key::A] {
                move_direction.x = -1.0;
            }

            // Normalizing Vec(0,0) causes NaN values.
            if move_direction.magnitude() > 0.1 {
                move_direction.normalize();
            }

            player_action_map.desired_move_direction = move_direction;
            player_action_map.desired_heading_direction =
                (input.mouse_position - transform.position).normalize();
        }
    }
}
