use super::*;
use specs::prelude::*;

pub struct MartineActionSystem;

impl MartineActionSystem {
    pub const MOVE_SPEED: f32 = 5.0;
    pub const FIRE_RATE: f32 = 0.2;
}

impl<'a> System<'a> for MartineActionSystem {
    type SystemData = (
        WriteStorage<'a, MarineActionMap>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, RigidBody>,
        Entities<'a>,
        ReadExpect<'a, Input>,
        Read<'a, LazyUpdate>,
    );

    fn run(
        &mut self,
        (mut action_maps, mut transforms, mut rigidbodies, entities, input, updater): Self::SystemData,
    ) {
        for (action_map, transform, rigidbody, entity) in (
            &mut action_maps,
            &mut transforms,
            &mut rigidbodies,
            &entities,
        )
            .join()
        {
            rigidbody.velocity = action_map.desired_move_direction
                * MartineActionSystem::MOVE_SPEED
                * action_map.speed_modifier;
            let heading = action_map.desired_heading_direction;
            transform.rotation = arctan2(heading.y, heading.x) + PI * 0.5;

            action_map.shoot_cooldown -= input.dt;
            let shoot = action_map.shoot && action_map.shoot_cooldown < 0.0;
            if shoot {
                action_map.shoot_cooldown =
                    MartineActionSystem::FIRE_RATE / action_map.fire_rate_modifier;
                let bullet = entities.create();
                {
                    updater.insert(
                        bullet,
                        Transform {
                            position: transform.position + heading,
                            rotation: transform.rotation,
                            parent: None,
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
                            layer: 4,
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
                    updater.insert(muzzle_flash, MuzzleFlash { time_to_live: 0.1 });
                    updater.insert(
                        muzzle_flash,
                        Transform {
                            position: Vector2::new(0.0, -1.25),
                            rotation: 0.0,
                            parent: Some(entity),
                        },
                    );
                    updater.insert(
                        muzzle_flash,
                        Sprite {
                            sprite: SpriteType::MuzzleFlash,
                            size: Vector2::new(2.0, 2.0),
                            layer: 5,
                        },
                    );
                }
            }
        }
    }
}
