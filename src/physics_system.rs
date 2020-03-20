use super::*;
use specs::*;

pub struct PhysicsSystem;

impl<'a> System<'a> for PhysicsSystem {
    type SystemData = (ReadStorage<'a, RigidBody>, WriteStorage<'a, Transform>);

    fn run(&mut self, (rigid_bodies, mut transforms): Self::SystemData) {
        for (rigid_body, transform) in (&rigid_bodies, &mut transforms).join() {
            transform.position += rigid_body.velocity;
        }
    }
}
