use super::*;
use specs::*;

pub struct MuzzleFlashSystem;

impl<'a> System<'a> for MuzzleFlashSystem {
    type SystemData = (
        WriteStorage<'a, MuzzleFlash>,
        Entities<'a>,
        ReadExpect<'a, Input>
    );

    fn run(&mut self, (mut flashes, entities, input): Self::SystemData) {
        for (flash, entity) in (&mut flashes, &entities).join() {
            flash.time_to_live -= input.dt;
            if flash.time_to_live <= 0.0 {
                let _ = entities.delete(entity);
            }
        }
    }
}