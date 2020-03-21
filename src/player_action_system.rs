use super::*;
use specs::prelude::*;

pub struct PlayerActionSystem;

impl PlayerActionSystem {
    pub const MOVE_SPEED: f32 = 5.0;
}

impl<'a> System<'a> for PlayerActionSystem {
    type SystemData = (
        ReadStorage<'a, PlayerActionMap>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, RigidBody>,
        Entities<'a>,
        Read<'a, LazyUpdate>,
    );

    fn run(
        &mut self,
        (action_maps, mut transforms, mut rigidbodies, entities, updater): Self::SystemData,
    ) {
        for (action_map, transform, rigidbody) in
            (&action_maps, &mut transforms, &mut rigidbodies).join()
        {
            rigidbody.velocity = action_map.desired_move_direction * PlayerActionSystem::MOVE_SPEED;
            let heading = action_map.desired_heading_direction;
            transform.rotation = arctan2(heading.y, heading.x) + PI * 0.5;

            if action_map.shoot {
                let bullet = entities.create();
                {
                    updater.insert(
                        bullet,
                        Transform {
                            position: transform.position + heading,
                            rotation: transform.rotation,
                        },
                    );
                    updater.insert(
                        bullet,
                        RigidBody {
                            velocity: heading * 30.0,
                            spin: 0.0,
                        },
                    );
                    updater.insert(
                        bullet,
                        Sprite {
                            sprite: SpriteType::Bullet,
                            size: Vector2::new(2.0, 2.0),
                        },
                    );
                    updater.insert(
                        bullet,
                        Collider {
                            collider_type: ColliderType::Sphere,
                            radius: 0.1,
                            is_trigger: true,
                            size: Vector2::zeros(),
                        },
                    );
                    updater.insert(bullet, Bullet {});
                }
                let muzzle_flash = entities.create();
                {
                    updater.insert(muzzle_flash, MuzzleFlash { time_to_live: 0.01 });
                    updater.insert(
                        muzzle_flash,
                        Transform {
                            position: transform.position + 1.5 * heading * 0.9,
                            rotation: transform.rotation,
                        },
                    );
                    updater.insert(
                        muzzle_flash,
                        Sprite {
                            sprite: SpriteType::MuzzleFlash,
                            size: Vector2::new(2.0, 2.0),
                        },
                    );
                }
            }
        }
    }
}
