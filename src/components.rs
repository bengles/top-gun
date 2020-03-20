use super::*;
use specs::{Component, DenseVecStorage};

#[derive(Component)]
pub struct Transform {
    pub position: Vector2,
    pub rotation: f32,
}

#[derive(Component)]
pub struct RigidBody {
    pub velocity: Vector2,
    pub spin: f64,
}

#[derive(Component)]
pub struct Sprite {
    pub size: Vector2,
    pub sprite: SpriteType,
}
