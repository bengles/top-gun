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
    pub network_messages: Vec<NetworkMessage>,
    pub is_host: bool,
}

impl<'a, 'b> Game<'a, 'b> {
    pub fn new(assets: Assets, is_host: bool) -> Game<'a, 'b> {
        // use this to initalize the game struct.
        let mut world = World::new();

        world.register::<Transform>();
        world.register::<RigidBody>();
        world.register::<Collider>();
        world.register::<Sprite>();
        world.register::<MarineActionMap>();
        world.register::<Bullet>();
        world.register::<MuzzleFlash>();
        world.register::<Scroll>();
        world.register::<Player>();
        world.register::<AI>();
        world.register::<Network>();

        let dispatcher = DispatcherBuilder::new()
            .with(PhysicsSystem, "physics_system", &[])
            .with(
                InputToMarineActionSystem,
                "input_to_marine_action_system",
                &[],
            )
            .with(MartineActionSystem, "player_action_system", &[])
            .with(BulletSystem, "bullet_system", &[])
            .with(MuzzleFlashSystem, "muzzle_flash_system", &[])
            .with(ScrollSystem, "scroll_system", &[])
            .with(AiMarineActionSystem, "ai_marine_action_system", &[])
            // .with(
            //     NetworkMarineActionSystem,
            //     "network_marine_action_system",
            //     &[],
            // )
            // .with(
            //     NetworkTransformSyncSystem,
            //     "network_transform_sync_system",
            //     &[],
            // )
            .build();

        Game {
            state: GameState::Init,
            world: world,
            dispatcher: dispatcher,
            assets: assets,
            input: Input::default(),
            network_messages: vec![],
            is_host: is_host,
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
        // Water
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

        // Background
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

        // Player
        let id = if self.is_host { 0 } else { 1 };
        let sprite = if self.is_host {
            SpriteType::Player2
        } else {
            SpriteType::Player1
        };
        self.world
            .create_entity()
            .with(Transform {
                position: Vector2::new(0.0, 0.0),
                rotation: 0.0,
                parent: None,
            })
            .with(Sprite {
                size: Vector2::new(1.0, 1.0),
                sprite: sprite,
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
            .with(MarineActionMap {
                shoot: false,
                desired_move_direction: Vector2::zeros(),
                desired_heading_direction: Vector2::zeros(),
                shoot_cooldown: 0.0,
                fire_rate_modifier: 1.0,
                speed_modifier: 1.0,
            })
            .with(Player {})
            .with(Network { id: id })
            .build();

        // Enemy
        let id = if self.is_host { 1 } else { 0 };
        let sprite = if self.is_host {
            SpriteType::Player1
        } else {
            SpriteType::Player2
        };
        self.world
            .create_entity()
            .with(Transform {
                position: Vector2::new(5.0, 0.0),
                rotation: 0.0,
                parent: None,
            })
            .with(Sprite {
                size: Vector2::new(1.0, 1.0),
                sprite: sprite,
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
            .with(MarineActionMap {
                shoot: false,
                desired_move_direction: Vector2::new(0.0, 0.0),
                desired_heading_direction: Vector2::new(-1.0, 0.0),
                shoot_cooldown: 0.0,
                fire_rate_modifier: 1.0,
                speed_modifier: 1.0,
            })
            .with(Network { id: id })
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
