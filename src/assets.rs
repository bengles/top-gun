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
        Assets { sprites: sprites }
    }
}

#[derive(Eq, PartialEq, Hash)]
pub enum SpriteType {
    Defense,
}
