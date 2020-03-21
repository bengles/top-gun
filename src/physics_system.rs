use super::*;
use components::ColliderType::*;
use specs::*;

pub struct PhysicsSystem;

impl<'a> System<'a> for PhysicsSystem {
    type SystemData = (
        ReadStorage<'a, RigidBody>,
        ReadStorage<'a, Collider>,
        WriteStorage<'a, Transform>,
        Entities<'a>,
        WriteExpect<'a, CollisionPairs>,
        ReadExpect<'a, Input>,
    );

    fn run(
        &mut self,
        (rigid_bodies, colliders, mut transforms, entities, mut collision_pairs, input): Self::SystemData,
    ) {
        collision_pairs.pairs.clear();

        let mut other_entities: Vec<(&RigidBody, &Collider, Entity, Transform)> = vec![];
        for (rigid_body, collider, entity, transform) in
            (&rigid_bodies, &colliders, &entities, &transforms).join()
        {
            other_entities.push((rigid_body, collider, entity, transform.clone()));
        }

        for (rigid_body, collider, entity, transform) in
            (&rigid_bodies, &colliders, &entities, &mut transforms).join()
        {
            if rigid_body.velocity.x == 0.0 && rigid_body.velocity.y == 0.0 {
                continue;
            }

            transform.position += rigid_body.velocity * input.dt;

            for (_other_rigid_body, other_collider, other_entity, other_transform) in
                &other_entities
            {
                if !std::ptr::eq(collider, *other_collider)
                    && are_colliding(collider, other_collider, transform, other_transform)
                {
                    if collider.is_trigger || other_collider.is_trigger {
                        collision_pairs.pairs.push((entity, *other_entity));
                    } else {
                        treat_collision(
                            rigid_body,
                            collider,
                            other_collider,
                            transform,
                            other_transform,
                        );
                    }
                }
            }
        }
    }
}

pub struct CollisionPairs {
    pub pairs: Vec<(Entity, Entity)>,
}

fn are_colliding(
    first_collider: &Collider,
    second_collider: &Collider,
    first_transform: &Transform,
    second_transform: &Transform,
) -> bool {
    if first_collider.collider_type == second_collider.collider_type {
        if first_collider.collider_type == Sphere {
            let distance = (first_transform.position - second_transform.position).magnitude();
            if distance < first_collider.radius + second_collider.radius {
                return true;
            }
        }
    }
    false
}

fn treat_collision(
    rigid_body: &RigidBody,
    first_collider: &Collider,
    second_collider: &Collider,
    first_transform: &mut Transform,
    second_transform: &Transform,
) {
    if first_collider.collider_type == second_collider.collider_type {
        if first_collider.collider_type == Sphere {
            let distance = (first_transform.position - second_transform.position).magnitude();
            let back_distance =
                ((first_collider.radius + second_collider.radius).powf(2.0) - distance).sqrt();
            let move_length = rigid_body.velocity.magnitude();
            first_transform.position = Vector2::new(
                first_transform.position.x - back_distance * (rigid_body.velocity.x / move_length),
                first_transform.position.y - back_distance * (rigid_body.velocity.y / move_length),
            );
        }
    }
}
