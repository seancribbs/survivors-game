use std::collections::HashMap;

use bevy::prelude::*;

use crate::schedule::InGame;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, detect_collisions.in_set(InGame::CollisionDetection));
        // .add_systems(Update, knock_back_player);
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

// fn knock_back_player(
//     mut player: Query<(&Collider, &Transform, &mut Velocity), With<Player>>,
//     transforms: Query<&Transform>,
//     time: Res<Time>,
// ) {
//     let Ok((collider, transform, mut velocity)) = player.get_single_mut() else {
//         return;
//     };

//     let mut knock_back = Vec3::ZERO;
//     for collision in collider.collisions.iter() {
//         let enemy_transform = transforms
//             .get(*collision)
//             .expect("Collided with entity that has no transform");
//         let direction = (transform.translation - enemy_transform.translation)
//             .try_normalize()
//             .unwrap_or(Vec3::NEG_X);
//         let acceleration = direction * 30.;
//         knock_back += acceleration;
//     }
//     if knock_back.length_squared() > f32::EPSILON {
//         info!("Knocking back player with acceleration {knock_back:?}");
//     }
//     velocity.accelerate(knock_back, time.elapsed_seconds());
// }
