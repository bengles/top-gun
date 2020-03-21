use super::*;

pub fn arctan2(y: f32, x: f32) -> f32 {
    if x > 0.0 {
        return (y / x).atan();
    }
    if x < 0.0 && y >= 0.0 {
        return (y / x).atan() + PI;
    }
    if x < 0.0 && y < 0.0 {
        return (y / x).atan() - PI;
    }
    if x == 0.0 && y > 0.0 {
        return PI * 0.5;
    }
    if x == 0.0 && y < 0.0 {
        return PI * 0.5 * -1.0;
    }
    0.0
}

pub fn is_out_of_bounds(world_size: Vector2, position: Vector2, border_factor: f32) -> bool {
    if position.x < -world_size.x * border_factor {
        return true;
    }
    if position.x > world_size.x * border_factor {
        return true;
    }
    // world_size.y is negative.
    if position.y < world_size.y * border_factor {
        return true;
    }
    if position.y > -world_size.y * border_factor {
        return true;
    }
    false
}

pub const PI: f32 = 3.141592;
