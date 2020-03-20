use std::collections::hash_map::HashMap;
use ggez::{graphics, Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler};
use specs::{ReadStorage, join::Join};

mod components;

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
        .add_resource_path("resources")
        .build()
		.expect("aieee, could not create ggez context!");

    let mut game = TopGun::new(&mut ctx);

    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e)
    }
}

struct TopGun<'a, 'b> {
    pub game: Game<'a, 'b>,
    pub sprites: HashMap<&'a str, graphics::Image>,
}

impl<'a, 'b> TopGun<'a, 'b> {
    pub fn new(ctx: &mut Context) -> TopGun<'a, 'b> {
        let sprite = "/sprites/defense_sphere.png";
        let test_sprite = graphics::Image::new(ctx, sprite).unwrap();
        let mut sprites = HashMap::<& str, graphics::Image>::new();
        sprites.insert(sprite, test_sprite);

        TopGun {
            game: Game::new(),
            sprites: sprites,
        }
    }

    pub fn update_view_matrix(&mut self, ctx: &mut Context) {
        let window_size = graphics::size(ctx);
        let view_matrix = Matrix4::new_translation(&ggez::nalgebra::Vector3::new(
            window_size.0 as f32 * 0.5,
            window_size.1 as f32 * 0.5,
            0.0,
        )) * Matrix4::new_nonuniform_scaling(&ggez::nalgebra::Vector3::new(
            window_size.1 as f32 * 0.5,
            window_size.1 as f32 * 0.5,
            1.0,
        ));

        let origin = Point2::origin();
        let world_to_screen = view_matrix
            * Matrix4::new_nonuniform_scaling(&ggez::nalgebra::Vector3::new(0.1, -0.1, 1.0))
            * Matrix4::new_translation(&ggez::nalgebra::Vector3::new(-origin.x, -origin.y, 0.0));

        graphics::set_transform(ctx, world_to_screen);
        graphics::apply_transformations(ctx).unwrap();
    }
}

impl<'a, 'b> EventHandler for TopGun<'a, 'b> {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);
        self.update_view_matrix(ctx);

        // Render all sprite objects
        let (sprites, transforms): (ReadStorage<components::Sprite>, ReadStorage<components::Transform>) = self.game.world.system_data();
        for (sprite, transform) in (&sprites, &transforms).join() {
            let p = graphics::DrawParam::new()
                .dest(Point2::new(
                    transform.position.x - sprite.size.x * 0.5,
                    transform.position.y - sprite.size.y * 0.5,
                ))
                .color([1.0, 1.0, 1.0, 1.0].into());

            let rectangle = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                graphics::Rect::new(0.0, 0.0, sprite.size.x, sprite.size.y),
                [1.0, 1.0, 1.0, 1.0].into(),
            )?;
            graphics::draw(ctx, &rectangle, p)?;
        }

        graphics::present(ctx)
    }
}
