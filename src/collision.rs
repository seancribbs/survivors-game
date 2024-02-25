use std::collections::HashMap;

use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

use crate::{
    ghost::Ghost,
    player::{Dagger, Player},
    schedule::InGame,
};

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, detect_collisions.in_set(InGame::CollisionDetection))
            .add_systems(Update, setup_collision_gizmos)
            .add_systems(
                Update,
                (
                    handle_collisions::<Player>,
                    handle_collisions::<Ghost>,
                    handle_collisions::<Dagger>,
                )
                    .in_set(InGame::ProcessCombat),
            )
            .add_event::<CollisionEvent>();
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

#[derive(Component, Debug)]
pub struct CollisionDamage {
    pub amount: u32,
}

impl CollisionDamage {
    pub fn new(amount: u32) -> Self {
        Self { amount }
    }
}

#[derive(Debug, Event)]
pub struct CollisionEvent {
    pub entity: Entity,
    pub collided_with: Entity,
}

impl CollisionEvent {
    pub fn new(entity: Entity, collided_with: Entity) -> Self {
        Self {
            entity,
            collided_with,
        }
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

fn handle_collisions<T: Component>(
    mut events: EventWriter<CollisionEvent>,
    query: Query<(Entity, &Collider), With<T>>,
) {
    for (entity, collider) in query.iter() {
        for collided_with in collider.collisions.iter() {
            events.send(CollisionEvent::new(entity, *collided_with));
        }
    }
}

fn setup_collision_gizmos(
    mut commands: Commands,
    query: Query<(Entity, &Collider), Added<Collider>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if !cfg!(feature = "gizmos") {
        return;
    }
    let color = Color::LIME_GREEN.with_a(0.50);
    let center_color = Color::CYAN;
    for (entity, collider) in query.iter() {
        commands.entity(entity).with_children(|children| {
            children.spawn(MaterialMesh2dBundle {
                mesh: meshes.add(shape::Quad::new(collider.size).into()).into(),
                material: materials.add(ColorMaterial::from(color)),
                transform: Transform::from_translation(collider.offset.extend(100.0)),
                ..default()
            });
            children.spawn(MaterialMesh2dBundle {
                mesh: meshes.add(shape::Quad::new(Vec2::ONE).into()).into(),
                material: materials.add(ColorMaterial::from(center_color)),
                transform: Transform::from_translation(collider.offset.extend(101.0)),
                ..default()
            });
        });
    }
}
