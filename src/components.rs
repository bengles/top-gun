use super::*;
use specs::{Component, DenseVecStorage};

#[derive(Component)]
pub struct Transform {
    pub position: Vector2,
    pub rotation: f32,
}

#[derive(Component)]
pub struct Sprite {
    pub size: Vector2,
    pub sprite: SpriteType,
}

#[derive(Component)]
pub struct PlayerActionMap {
    pub shoot: bool,
    pub desired_move_direction: Vector2,
    pub desired_heading_direction: Vector2,
}
