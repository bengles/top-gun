use ggez::{graphics, Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler};

// Define usual 2d data structs.
pub type Point2 = ggez::nalgebra::Point2<f32>;
pub type Vector2 = ggez::nalgebra::Vector2<f32>;
pub type Matrix4 = ggez::nalgebra::Matrix4<f32>;

pub mod game;
use game::*;

fn main() {
    let (mut ctx, mut event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
		.build()
		.expect("aieee, could not create ggez context!");

    let mut game = TopGun::new();

    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e)
    }
}

struct TopGun<'a, 'b> {
    pub game: Game<'a, 'b>,
}

impl<'a, 'b> TopGun<'a, 'b> {
    pub fn new() -> TopGun<'a, 'b> {
        TopGun {
            game: Game::new(),
        }
    }
}

impl<'a, 'b> EventHandler for TopGun<'a, 'b> {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);
        // Draw code here...
        graphics::present(ctx)
    }
}