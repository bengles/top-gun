use super::*;
use specs::*;

pub struct PhysicsSystem;

impl<'a> System<'a> for PhysicsSystem {
    type SystemData = (
        ReadStorage<'a, RigidBody>,
        ReadStorage<'a, Collider>,
        WriteStorage<'a, Transform>,
    );

    fn run(&mut self, (rigid_bodies, colliders, mut transforms): Self::SystemData) {
        for (rigid_body, collider, transform) in (&rigid_bodies, &colliders, &mut transforms).join() {
            transform.position += rigid_body.velocity;
        }
    }
}
