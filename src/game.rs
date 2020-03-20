use components::*;
use super::*;
use specs::*;

pub struct Game<'a, 'b> {
    pub world: World,
    pub dispatcher: Dispatcher<'a, 'b>,
}

impl<'a, 'b> Game<'a, 'b> {
    pub fn new() -> Game<'a, 'b> {
        // use this to initalize the game struct.

        let mut world = World::new();

        world.register::<Transform>();
        world.register::<Sprite>();

        world.create_entity()
        .with(Transform {
            position: Vector2::new(0.0, 0.0),
            rotation: 0.0
        })
        .with(Sprite {
            size: Vector2::new(1.0, 1.0),
            sprite: "defense_sphere".to_owned(),
        })
        .build();

        let dispatcher = DispatcherBuilder::new().build();

        Game {
            world: world,
            dispatcher: dispatcher,
        }
    }

    pub fn update(&mut self) {
        // update loop of the game.
        self.dispatcher.run_now(&self.world);
    }
}