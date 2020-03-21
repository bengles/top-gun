use super::*;
use specs::*;

pub struct BulletSystem;

impl<'a> System<'a> for BulletSystem {
    type SystemData = (
        ReadStorage<'a, Transform>,
        ReadStorage<'a, Bullet>,
        Entities<'a>,
        ReadExpect<'a, Input>,
    );

    fn run(&mut self, (transforms, bullets, entities, input): Self::SystemData) {
        for (tranform, _bullet, entity) in (&transforms, &bullets, &entities).join() {
            if utils::is_out_of_bounds(input.world_size, tranform.position, 1.05) {
                let _ = entities.delete(entity);
            }
        }
    }
}
