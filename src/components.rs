use super::*;
use specs::{prelude::*, Component, DenseVecStorage};

#[derive(Debug, Component, Copy, Clone, Serialize, Deserialize)]
pub struct Transform {
    pub position: Vector2,
    pub rotation: f32,
    #[serde(skip)]
    pub parent: Option<Entity>,
}

impl Default for Transform {
    fn default() -> Transform {
        Transform {
            position: Vector2::zeros(),
            rotation: 0.0,
            parent: None,
        }
    }
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

#[derive(Debug, Component, Copy, Clone, Serialize, Deserialize)]
pub struct MarineActionMap {
    pub shoot: bool,
    pub desired_move_direction: Vector2,
    pub desired_heading_direction: Vector2,
    pub shoot_cooldown: f32,
    pub fire_rate_modifier: f32,
    pub speed_modifier: f32,
}

impl Default for MarineActionMap {
    fn default() -> MarineActionMap {
        MarineActionMap {
            shoot: false,
            desired_move_direction: Vector2::zeros(),
            desired_heading_direction: Vector2::new(1.0, 0.0),
            shoot_cooldown: 0.0,
            fire_rate_modifier: 1.0,
            speed_modifier: 1.0,
        }
    }
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

#[derive(Component)]
pub struct Network {
    pub id: u32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct NetworkMessage {
    pub id: u32,
    pub message_type: u32,
    pub transform: Transform,
    pub action_map: MarineActionMap,
}
