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

pub fn rotate_vector(vector: Vector2, angle: f32) -> Vector2 {
    let px = vector.x * angle.cos() - vector.y * angle.sin();
    let py = vector.x * angle.sin() + vector.y * angle.cos();
    Vector2::new(px, py)
}

pub fn sync_transforms(world: &mut World) -> Vec<(Vector2, f32)> {
    let mut parent_transforms = vec![];
    let (entities, mut transforms, _sprites): (
        Entities,
        WriteStorage<Transform>,
        ReadStorage<Sprite>,
    ) = world.system_data();
    for entity in (&entities).join() {
        if let Some(transform) = &transforms.get(entity) {
            if let Some(parent_entity) = transform.parent {
                if let Some(parent_transform) = &transforms.get(parent_entity) {
                    parent_transforms.push((parent_transform.position, parent_transform.rotation));
                }
            }
        }
    }

    let mut world_transforms = vec![];
    let mut i: usize = 0;
    for (_entity, transform) in (&entities, &mut transforms).join() {
        let mut position = Vector2::zeros();
        let mut rotation = 0.0;
        if transform.parent.is_some() {
            position += parent_transforms[i].0;
            rotation += parent_transforms[i].1;
            i += 1;
        }
        world_transforms.push((
            position + rotate_vector(transform.position, rotation),
            rotation + transform.rotation,
        ));
    }
    world_transforms
}

pub const PI: f32 = 3.141592;
pub const TO_RADIANS: f32 = (1.0 / 360.0) * 2.0 * PI;
