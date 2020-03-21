use super::*;
use specs::*;

use physics_system::*;

pub enum GameState {
    Init,
    Play,
}

pub struct Game<'a, 'b> {
    pub state: GameState,
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
        world.register::<Scroll>();

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
            .with(ScrollSystem, "scroll_system", &[])
            .build();

        Game {
            state: GameState::Init,
            world: world,
            dispatcher: dispatcher,
            assets: assets,
            input: Input::default(),
        }
    }

    pub fn update(&mut self) {
        // update loop of the game.
        self.world.insert(self.input.clone());

        match self.state {
            GameState::Init => self.init(),
            GameState::Play => self.dispatcher.run_now(&self.world),
        }
        self.world.maintain();
    }
}

impl Game<'_, '_> {
    pub fn init(&mut self) {
        let water_size = Vector2::new(16.0, 9.0) * 2.2;
        for i in 0..2 {
            self.world
                .create_entity()
                .with(Transform {
                    position: i as f32 * Vector2::new(0.0, -water_size.y),
                    rotation: 0.0,
                    parent: None,
                })
                .with(RigidBody {
                    velocity: Vector2::new(0.0, 20.0),
                    spin: 0.0,
                })
                .with(Sprite {
                    size: water_size,
                    sprite: SpriteType::Water,
                    layer: 0,
                })
                .with(Collider {
                    collider_type: ColliderType::Rectangle,
                    size: Vector2::zeros(),
                    radius: 0.0,
                    is_trigger: true,
                })
                .with(Scroll {})
                .build();
        }

        self.world
            .create_entity()
            .with(Transform {
                position: Vector2::new(0.0, 0.0),
                rotation: 0.0,
                parent: None,
            })
            .with(Sprite {
                size: Vector2::new(16.0, 9.0) * 2.2,
                sprite: SpriteType::Background,
                layer: 1,
            })
            .build();

        self.world
            .create_entity()
            .with(Transform {
                position: Vector2::new(0.0, 0.0),
                rotation: 0.0,
                parent: None,
            })
            .with(Sprite {
                size: Vector2::new(1.0, 1.0),
                sprite: SpriteType::Player1,
                layer: 5,
            })
            .with(RigidBody {
                velocity: Vector2::new(0.0, 0.0),
                spin: 0.0,
            })
            .with(Collider {
                collider_type: ColliderType::Sphere,
                radius: 0.5,
                size: Vector2::new(0.0, 0.0),
                is_trigger: false,
            })
            .with(PlayerActionMap {
                shoot: false,
                desired_move_direction: Vector2::zeros(),
                desired_heading_direction: Vector2::zeros(),
                shoot_cooldown: 0.0,
            })
            .build();

        self.world
            .create_entity()
            .with(Transform {
                position: Vector2::new(0.0, 5.0),
                rotation: 0.0,
                parent: None,
            })
            .with(Sprite {
                size: Vector2::new(1.0, 1.0),
                sprite: SpriteType::Defense,
                layer: 2,
            })
            .with(RigidBody {
                velocity: Vector2::new(0.0, 0.0),
                spin: 0.0,
            })
            .with(Collider {
                collider_type: ColliderType::Sphere,
                radius: 0.5,
                size: Vector2::new(0.0, 0.0),
                is_trigger: false,
            })
            .build();

        let collision_pairs: CollisionPairs = CollisionPairs { pairs: vec![] };
        self.world.insert(collision_pairs);

        self.state = GameState::Play;
    }
}
