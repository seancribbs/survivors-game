use bevy::prelude::*;

use crate::{collision::Collider, ghost::Ghost, levels::Wall, player::Player, schedule::InGame};

pub struct MovementPlugin;

#[derive(Component, Debug, Default)]
pub struct Velocity {
    value: Vec3,
}

impl Velocity {
    pub fn from_direction_speed(direction: Vec3, speed: f32) -> Self {
        Self {
            value: direction.normalize_or_zero() * speed,
        }
    }

    pub fn new(x: f32, y: f32) -> Self {
        Self {
            value: Vec3::new(x, y, 0.),
        }
    }

    pub fn change_direction_speed(&mut self, direction: Vec3, speed: f32) {
        self.value = direction.normalize_or_zero() * speed;
    }
}

#[derive(Bundle, Debug, Default)]
pub struct MovementBundle {
    pub velocity: Velocity,
}

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_position,
                keep_inside_walls::<Player>,
                keep_inside_walls::<Ghost>,
            )
                .chain()
                .in_set(InGame::EntityUpdates),
        );
    }
}

fn update_position(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation += velocity.value * time.delta_seconds();
    }
}

fn keep_inside_walls<T: Component>(
    mut query: Query<(&mut Transform, &Collider), (With<T>, Without<Wall>)>,
    walls: Query<(&Transform, &Collider), With<Wall>>,
) {
    // Note: This would be easier and more consistent if we use integer pixel dimensions and not FP
    for (mut transform, collider) in query.iter_mut() {
        let object_rect = collider.to_rect_at(&transform);
        for collision in collider.collisions.iter() {
            if let Ok((wall_transform, wall_collider)) = walls.get(*collision) {
                let wall_rect = wall_collider.to_rect_at(wall_transform);
                let overlap = object_rect.intersect(wall_rect);

                // We assume that the overlapping dimensions will be largest
                // in the direction that we not colliding in.
                let base_push = if overlap.width() < overlap.height() {
                    Vec2::new(overlap.width(), 0.)
                } else {
                    Vec2::new(0., overlap.height())
                };
                let push_away = base_push * (object_rect.center() - wall_rect.center()).signum();
                transform.translation += push_away.extend(0.);
            }
        }
    }
}
