use super::*;
use specs::*;

use physics_system::*;

pub struct Game<'a, 'b> {
    pub world: World,
    pub dispatcher: Dispatcher<'a, 'b>,
    pub assets: Assets,
    pub input: Input,
}

impl<'a, 'b> Game<'a, 'b> {
    pub fn new(assets: Assets) -> Game<'a, 'b> {
        // use this to initalize the game struct.
        let mut world = World::new();

        world.register::<Transform>();
        world.register::<RigidBody>();
        world.register::<Collider>();
        world.register::<Sprite>();
        world.register::<PlayerActionMap>();
        world.register::<Bullet>();
        world.register::<MuzzleFlash>();

        world
            .create_entity()
            .with(Transform {
                position: Vector2::new(0.0, 0.0),
                rotation: 0.0,
            })
            .with(Sprite {
                size: Vector2::new(16.0, 9.0) * 2.2,
                sprite: SpriteType::Background,
            })
            .build();

        world
            .create_entity()
            .with(Transform {
                position: Vector2::new(0.0, 0.0),
                rotation: 0.0,
            })
            .with(Sprite {
                size: Vector2::new(1.0, 1.0),
                sprite: SpriteType::Player1,
            })
            .with(RigidBody {
                velocity: Vector2::new(0.0, 0.0),
                spin: 0.0,
            })
            .with(Collider {
                collider_type: ColliderType::Sphere,
                radius: 0.0,
                size: Vector2::new(0.0, 0.0),
                is_trigger: false,
            })
            .with(PlayerActionMap {
                shoot: false,
                desired_move_direction: Vector2::zeros(),
                desired_heading_direction: Vector2::zeros(),
            })
            .build();

        let dispatcher = DispatcherBuilder::new()
            .with(PhysicsSystem, "physics_system", &[])
            .with(
                InputToPlayerActionSystem,
                "input_to_player_action_system",
                &[],
            )
            .with(PlayerActionSystem, "player_action_system", &[])
            .with(BulletSystem, "bullet_system", &[])
            .with(MuzzleFlashSystem, "muzzle_flash_system", &[])
            .build();

        Game {
            world: world,
            dispatcher: dispatcher,
            assets: assets,
            input: Input::default(),
        }
    }

    pub fn update(&mut self) {
        // update loop of the game.
        self.world.insert(self.input.clone());
        self.dispatcher.run_now(&self.world);
        self.world.maintain();
    }
}
