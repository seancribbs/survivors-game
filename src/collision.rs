use std::collections::HashMap;

use bevy::prelude::*;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, detect_collisions);
    }
}

#[derive(Component, Debug)]
pub struct Collider {
    pub size: Vec2,
    pub collisions: Vec<Entity>,
}

impl Collider {
    pub fn new(size: Vec2) -> Self {
        Self {
            size,
            collisions: vec![],
        }
    }
}

fn detect_collisions(mut query: Query<(Entity, &Transform, &mut Collider)>) {
    let mut collisions: HashMap<Entity, Vec<Entity>> = HashMap::new();
    // Detection
    for (entity_a, transform_a, collider_a) in query.iter() {
        let rect_a = Rect::from_center_size(transform_a.translation.truncate(), collider_a.size);

        for (entity_b, transform_b, collider_b) in query.iter() {
            // Don't collide with yourself
            if entity_b == entity_a {
                continue;
            }

            let rect_b =
                Rect::from_center_size(transform_b.translation.truncate(), collider_b.size);
            if !rect_a.intersect(rect_b).is_empty() {
                collisions.entry(entity_a).or_default().push(entity_b);
            }
        }
    }

    // Record
    for (entity, _, mut collider) in query.iter_mut() {
        collider.collisions = collisions.remove(&entity).unwrap_or_default();
    }
}
