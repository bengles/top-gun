use ggez::event::KeyCode;
use ggez::event::KeyMods;
use ggez::event::MouseButton;
use ggez::event::{self, EventHandler};
use ggez::{graphics, Context, ContextBuilder, GameResult};
use specs::{join::Join, ReadStorage};

mod assets;
mod bullet_system;
mod components;
mod input;
mod input_to_player_action_system;
mod muzzle_flash_system;
mod physics_system;
mod player_action_system;
mod utils;

use assets::*;
use bullet_system::*;
use components::*;
use input::*;
use input_to_player_action_system::*;
use muzzle_flash_system::*;
use player_action_system::*;
use utils::*;

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
        Err(e) => println!("Error occured: {}", e),
    }
}

struct TopGun<'a, 'b> {
    pub game: Game<'a, 'b>,
    pub input: Input,
    pub screen_to_world: Matrix4,
}

impl<'a, 'b> TopGun<'a, 'b> {
    pub fn new(ctx: &mut Context) -> TopGun<'a, 'b> {
        let assets = Assets::load_assets(ctx);
        TopGun {
            game: Game::new(assets),
            input: Input::default(),
            screen_to_world: Matrix4::identity(),
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

        self.screen_to_world = world_to_screen.try_inverse().unwrap();

        graphics::set_transform(ctx, world_to_screen);
        graphics::apply_transformations(ctx).unwrap();
    }

    pub fn keycode_to_key(&mut self, keycode: KeyCode) -> Option<Key> {
        match keycode {
            KeyCode::W => Some(Key::W),
            KeyCode::S => Some(Key::S),
            KeyCode::D => Some(Key::D),
            KeyCode::A => Some(Key::A),
            _ => None,
        }
    }

    fn screen_to_world(&self, pt: Vector2) -> Vector2 {
        let pt =
            self.screen_to_world * ggez::nalgebra::Vector4::new(pt.x as f32, pt.y as f32, 0.0, 1.0);
        Vector2::new(pt.x, pt.y)
    }
}

impl<'a, 'b> EventHandler for TopGun<'a, 'b> {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.input.dt = ggez::timer::delta(ctx).as_secs_f32();
        let screen_size = graphics::size(ctx);
        let world_size = self.screen_to_world(Vector2::new(screen_size.0, screen_size.1));
        self.input.world_size = world_size;
        self.game.input = self.input.clone();
        self.game.update();
        self.input.reset();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);
        self.update_view_matrix(ctx);

        let mut layers = std::collections::HashMap::<u32, Vec<(&Sprite, &Transform)>>::new();

        // Render all sprite objects
        let (sprites, transforms): (ReadStorage<Sprite>, ReadStorage<Transform>) =
            self.game.world.system_data();
        for (sprite, transform) in (&sprites, &transforms).join() {
            if !layers.contains_key(&sprite.layer) {
                layers.insert(sprite.layer, vec![]);
            }
            layers
                .get_mut(&sprite.layer)
                .unwrap()
                .push((sprite, transform));
        }

        for layer in 0..16 {
            if let Some(layer_sprites) = layers.get(&layer) {
                for (sprite, transform) in layer_sprites {
                    let image = &self.game.assets.sprites[&sprite.sprite];
                    let p = graphics::DrawParam::new()
                        .dest(Point2::new(transform.position.x, transform.position.y))
                        .scale(Vector2::new(
                            sprite.size.x / image.width() as f32,
                            sprite.size.y / image.height() as f32,
                        ))
                        .rotation(transform.rotation)
                        .offset(Point2::new(0.5, 0.5))
                        .color([1.0, 1.0, 1.0, 1.0].into());
                    graphics::draw(ctx, image, p)?;
                }
            }
        }

        graphics::present(ctx)
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
        match button {
            MouseButton::Left => self.input.keys_down.insert(Key::Mouse1, true),
            MouseButton::Right => self.input.keys_down.insert(Key::Mouse2, true),
            _ => None,
        };

        match button {
            MouseButton::Left => self.input.keys_pressed.insert(Key::Mouse1, true),
            MouseButton::Right => self.input.keys_pressed.insert(Key::Mouse2, true),
            _ => None,
        };
    }
    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, _x: f32, _y: f32) {
        match button {
            MouseButton::Left => self.input.keys_up.insert(Key::Mouse1, true),
            MouseButton::Right => self.input.keys_up.insert(Key::Mouse2, true),
            _ => None,
        };

        match button {
            MouseButton::Left => self.input.keys_pressed.insert(Key::Mouse1, false),
            MouseButton::Right => self.input.keys_pressed.insert(Key::Mouse2, false),
            _ => None,
        };
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {
        self.input.mouse_position = self.screen_to_world(Vector2::new(x, y));
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        repeat: bool,
    ) {
        let optional_key = self.keycode_to_key(keycode);
        if let Some(key) = optional_key {
            if repeat {
                self.input.keys_pressed.insert(key, true);
            } else {
                self.input.keys_down.insert(key.clone(), true);
                self.input.keys_pressed.insert(key, true);
            }
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods) {
        let optional_key = self.keycode_to_key(keycode);
        if let Some(key) = optional_key {
            self.input.keys_up.insert(key.clone(), true);
            self.input.keys_pressed.insert(key, false);
        }
    }
}
