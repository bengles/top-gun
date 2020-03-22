use super::*;
use specs::*;

pub struct NetworkTransformSyncSystem;

impl<'a> System<'a> for NetworkTransformSyncSystem {
    type SystemData = (
        ReadStorage<'a, Player>,
        ReadStorage<'a, Network>,
        WriteStorage<'a, Transform>,
        ReadExpect<'a, Vec<NetworkMessage>>,
    );

    fn run(&mut self, (players, networks, mut transforms, network_messages): Self::SystemData) {
        for ((), network, transform) in (!&players, &networks, &mut transforms).join() {
            for message in network_messages.iter() {
                if message.message_type == 1 && network.id == message.id {
                    let network_transform: Transform = message.transform;
                    transform.position = network_transform.position;
                    transform.rotation = network_transform.rotation;
                }
            }
        }
    }
}
