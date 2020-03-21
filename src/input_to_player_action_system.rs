use super::*;
use specs::prelude::*;

pub struct InputToPlayerActionSystem;

impl<'a> System<'a> for InputToPlayerActionSystem {
    type SystemData = (
        ReadExpect<'a, Input>,
        ReadStorage<'a, Transform>,
        WriteStorage<'a, PlayerActionMap>,
    );

    fn run(&mut self, (input, transforms, mut player_action_maps): Self::SystemData) {
        for (transform, player_action_map) in (&transforms, &mut player_action_maps).join() {
            player_action_map.shoot_cooldown -= input.dt;
            player_action_map.shoot =
                input.keys_pressed[&Key::Mouse1] && player_action_map.shoot_cooldown < 0.0;
            if player_action_map.shoot {
                player_action_map.shoot_cooldown = PlayerActionSystem::FIRE_RATE;
            }
            let mut move_direction: Vector2 = Vector2::zeros();
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
