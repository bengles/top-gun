use super::*;
use specs::{prelude::*, Component, DenseVecStorage};

#[derive(Component, Clone)]
pub struct Transform {
    pub position: Vector2,
    pub rotation: f32,
    pub parent: Option<Entity>,
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

#[derive(PartialEq)]
pub enum ColliderType {
    Sphere,
    Rectangle,
}

#[derive(Component)]
pub struct Sprite {
    pub size: Vector2,
    pub sprite: SpriteType,
    pub layer: u32,
}

#[derive(Component)]
pub struct MarineActionMap {
    pub shoot: bool,
    pub desired_move_direction: Vector2,
    pub desired_heading_direction: Vector2,
    pub shoot_cooldown: f32,
    pub fire_rate_modifier: f32,
    pub speed_modifier: f32,
}

#[derive(Component)]
pub struct Bullet;

#[derive(Component)]
pub struct MuzzleFlash {
    pub time_to_live: f32,
}

#[derive(Component)]
pub struct Scroll;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct AI;
