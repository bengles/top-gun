use super::*;
use specs::*;

pub struct ScrollSystem;

impl<'a> System<'a> for ScrollSystem {
    type SystemData = (
        ReadStorage<'a, Scroll>,
        ReadStorage<'a, Sprite>,
        WriteStorage<'a, Transform>,
        ReadExpect<'a, Input>,
    );

    fn run(&mut self, (scrolls, sprites, mut transforms, input): Self::SystemData) {
        for (_scroll, sprite, transform) in (&scrolls, &sprites, &mut transforms).join() {
            if -input.world_size.y < (transform.position - Vector2::new(0.0, sprite.size.y * 0.5)).y
            {
                transform.position -= 2.0 * sprite.size.y * Vector2::new(0.0, 1.0);
            }
        }
    }
}
