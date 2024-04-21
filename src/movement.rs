use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::{
    collision::{Collider, CollisionEvent},
    enemies::Enemy,
    levels::Wall,
    player::Player,
    schedule::InGame,
};

pub struct MovementPlugin;

#[derive(Component, Debug, Clone, Default)]
pub struct Velocity {
    pub value: Vec3,
}

#[derive(Component, Debug, Clone, Default)]
pub struct Facing {
    pub value: Vec3,
}

impl Velocity {
    pub fn from_direction_speed(direction: Vec3, speed: f32) -> Self {
        Self {
            value: direction.normalize_or_zero() * speed,
        }
    }

    pub fn change_direction(&mut self, direction: Vec3) {
        let speed = self.value.length();
        self.change_direction_speed(direction, speed);
    }

    pub fn change_direction_speed(&mut self, direction: Vec3, speed: f32) {
        self.value = direction.normalize_or_zero() * speed;
    }
}

#[derive(Bundle, Debug, Clone, Default)]
pub struct MovementBundle {
    pub velocity: Velocity,
    pub facing: Facing,
}

impl From<&EntityInstance> for MovementBundle {
    fn from(value: &EntityInstance) -> Self {
        if let Ok(speed) = value.get_float_field("speed") {
            Self {
                velocity: Velocity::from_direction_speed(Vec3::X, *speed),
                ..Default::default()
            }
        } else {
            Default::default()
        }
    }
}

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_position,
                update_sprite_direction,
                keep_inside_walls::<Player>,
                keep_inside_walls::<Enemy>,
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

fn update_sprite_direction(mut query: Query<(&Facing, &mut TextureAtlasSprite)>) {
    for (facing, mut sprite) in query.iter_mut() {
        sprite.flip_x = facing.value.x.signum() < 0.;
    }
}

fn keep_inside_walls<T: Component>(
    mut events: EventReader<CollisionEvent>,
    mut query: Query<(&mut Transform, &Collider), (With<T>, Without<Wall>)>,
    walls: Query<(&Transform, &Collider), With<Wall>>,
) {
    for event in events.read() {
        let Ok((mut object_transform, object_collider)) = query.get_mut(event.entity) else {
            continue;
        };

        let Ok((wall_transform, wall_collider)) = walls.get(event.collided_with) else {
            continue;
        };

        let object_rect = object_collider.to_rect_at(&object_transform);
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
        object_transform.translation += push_away.extend(0.);
    }
}
