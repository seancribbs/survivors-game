use std::collections::HashMap;

use bevy::prelude::*;

use crate::schedule::InGame;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, detect_collisions.in_set(InGame::CollisionDetection));
    }
}

#[derive(Component, Debug)]
pub struct Collider {
    pub size: Vec2,
    pub offset: Vec2,
    pub collisions: Vec<Entity>,
}

impl Default for Collider {
    fn default() -> Self {
        Self::new(Vec2::ZERO)
    }
}

impl Collider {
    pub fn new(size: Vec2) -> Self {
        Self::with_size_and_offset(size, Vec2::ZERO)
    }

    pub fn with_size_and_offset(size: Vec2, offset: Vec2) -> Self {
        Self {
            size,
            offset,
            collisions: vec![],
        }
    }

    pub fn to_rect_at(&self, transform: &Transform) -> Rect {
        Rect::from_center_size(transform.translation.truncate() + self.offset, self.size)
    }
}

fn detect_collisions(mut query: Query<(Entity, &Transform, &mut Collider)>) {
    let mut collisions: HashMap<Entity, Vec<Entity>> = HashMap::new();
    // Detection
    for (entity_a, transform_a, collider_a) in query.iter() {
        let rect_a = collider_a.to_rect_at(transform_a);

        for (entity_b, transform_b, collider_b) in query.iter() {
            // Don't collide with yourself
            if entity_b == entity_a {
                continue;
            }

            let rect_b = collider_b.to_rect_at(transform_b);
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
