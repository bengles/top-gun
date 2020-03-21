use ggez::graphics::*;
use ggez::Context;
use std::collections::HashMap;

pub struct Assets {
    pub sprites: HashMap<SpriteType, Image>,
}

impl Assets {
    pub fn load_assets(ctx: &mut Context) -> Assets {
        let mut sprites = HashMap::new();
        sprites.insert(
            SpriteType::Defense,
            Image::new(ctx, "/sprites/defense_sphere.png").unwrap(),
        );
        sprites.insert(
            SpriteType::Player1,
            Image::new(ctx, "/sprites/player1.png").unwrap(),
        );
        sprites.insert(
            SpriteType::Player2,
            Image::new(ctx, "/sprites/player2.png").unwrap(),
        );
        sprites.insert(
            SpriteType::Background,
            Image::new(ctx, "/sprites/bridge_background.png").unwrap(),
        );
        sprites.insert(
            SpriteType::Bullet,
            Image::new(ctx, "/sprites/bullet.png").unwrap(),
        );
        sprites.insert(
            SpriteType::MuzzleFlash,
            Image::new(ctx, "/sprites/muzzle_flash.png").unwrap(),
        );
        sprites.insert(
            SpriteType::Water,
            Image::new(ctx, "/sprites/bridge_water.png").unwrap(),
        );
        Assets { sprites: sprites }
    }
}

#[derive(Eq, PartialEq, Hash)]
pub enum SpriteType {
    Defense,
    Background,
    Player1,
    Player2,
    Bullet,
    MuzzleFlash,
    Water,
}
