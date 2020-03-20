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
        world.register::<Sprite>();

        world
            .create_entity()
            .with(Transform {
                position: Vector2::new(0.0, 0.0),
                rotation: 0.0,
            })
            .with(Sprite {
                size: Vector2::new(1.0, 1.0),
                sprite: SpriteType::Defense,
            })
            .build();

        let world = World::new();
        let dispatcher = DispatcherBuilder::new()
            .with(PhysicsSystem, "physics_system", &[])
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
    }
}
