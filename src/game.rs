use specs::*;


pub struct Game<'a, 'b> {
    pub world: World,
    pub dispatcher: Dispatcher<'a, 'b>,
}

impl<'a, 'b> Game<'a, 'b> {
    pub fn new() -> Game<'a, 'b> {
        // use this to initalize the game struct.

        let world = World::new();
        let dispatcher = DispatcherBuilder::new().build();
        Game {
            world: world,
            dispatcher: dispatcher,
        }
    }

    pub fn update() {
        // update loop of the game.
    }
}