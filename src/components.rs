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
    pub spin: f32,
}

#[derive(Component)]
pub struct Collider {
    pub collider_type: ColliderType,
    pub radius: f32,
    pub size: Vector2,
    pub is_trigger: bool,
}

pub enum ColliderType {
    Sphere,
    Rectangle,
}

#[derive(Component)]
pub struct Sprite {
    pub size: Vector2,
    pub sprite: SpriteType,
}
