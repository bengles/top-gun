use super::*;
use specs::*;

pub struct NetworkMarineActionSystem;

impl<'a> System<'a> for NetworkMarineActionSystem {
    type SystemData = (
        ReadExpect<'a, Vec<NetworkMessage>>,
        ReadStorage<'a, Player>,
        WriteStorage<'a, MarineActionMap>,
        ReadStorage<'a, Network>,
    );

    fn run(&mut self, (network_messages, players, mut action_maps, networks): Self::SystemData) {
        for ((), action_map, _network) in (!&players, &mut action_maps, &networks).join() {
            for message in network_messages.iter() {
                if message.message_type == 0 {
                    let actions: MarineActionMap = serde_json::from_str(&message.payload).unwrap();
                    action_map.shoot = actions.shoot;
                    action_map.desired_move_direction = actions.desired_move_direction;
                    action_map.desired_heading_direction = actions.desired_heading_direction;
                }
            }
        }
    }
}
