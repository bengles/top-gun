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
        .window_setup(ggez::conf::WindowSetup {
            title: "Top Gun".to_owned(),
            samples: ggez::conf::NumSamples::Zero,
            vsync: false,
            icon: "".to_owned(),
            srgb: true,
        })
        .window_mode(ggez::conf::WindowMode {
            width: 1440.0,
            height: 810.0,
            maximized: false,
            fullscreen_type: ggez::conf::FullscreenType::Windowed,
            borderless: false,
            min_width: 1440.0,
            max_width: 1440.0,
            min_height: 810.0,
            max_height: 810.0,
            resizable: false,
        })
        .add_resource_path("resources/")
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