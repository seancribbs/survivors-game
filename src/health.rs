use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_ecs_ldtk::prelude::*;

use crate::{
    collision::{CollisionDamage, CollisionEvent},
    enemies::Enemy,
    player::{Dagger, Player},
    schedule::InGame,
};

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                take_damage::<Player, Enemy>,
                take_damage::<Enemy, Dagger>,
                despawn_dead_entities,
            )
                .chain()
                .in_set(InGame::ProcessCombat),
        )
        .add_systems(
            Update,
            (tick_damage_cooldown, create_health_bars, update_health_bars)
                .in_set(InGame::EntityUpdates),
        );
    }
}

#[derive(Component, Debug, Default)]
pub struct Health {
    pub amount: u32,
    pub total: u32,
    pub cooldown: Option<f32>,
}

#[derive(Component, Debug, Default)]
pub struct HealthBar;

#[derive(Component, Debug)]
pub struct HealthBarDisplay;

impl From<&EntityInstance> for Health {
    fn from(value: &EntityInstance) -> Self {
        if let Ok(v) = value.get_int_field("health") {
            Self {
                amount: *v as u32,
                total: *v as u32,
                ..Default::default()
            }
        } else {
            Default::default()
        }
    }
}

#[derive(Component, Debug)]
pub struct DamageCooldown {
    pub cooldown: Timer,
}

impl DamageCooldown {
    pub fn new(cooldown: f32) -> Self {
        let cooldown = Timer::from_seconds(cooldown, TimerMode::Once);
        Self { cooldown }
    }
}

impl Health {
    pub fn new(amount: u32) -> Self {
        Self {
            amount,
            total: amount,
            cooldown: None,
        }
    }

    pub fn with_damage_cooldown(amount: u32, cooldown: f32) -> Self {
        Self {
            amount,
            total: amount,
            cooldown: Some(cooldown),
        }
    }
}

fn take_damage<T: Component, E: Component>(
    mut commands: Commands,
    mut events: EventReader<CollisionEvent>,
    mut receiver: Query<&mut Health, (With<T>, Without<DamageCooldown>)>,
    damager: Query<&CollisionDamage, With<E>>,
) {
    for collision in events.read() {
        let Ok(mut health) = receiver.get_mut(collision.entity) else {
            continue;
        };

        let Ok(damage) = damager.get(collision.collided_with) else {
            continue;
        };

        health.amount = health.amount.saturating_sub(damage.amount);

        if let Some(duration) = health.cooldown.as_ref() {
            commands
                .entity(collision.entity)
                .insert(DamageCooldown::new(*duration));
        }
    }
}

fn tick_damage_cooldown(
    mut commands: Commands,
    mut query: Query<(Entity, &mut DamageCooldown)>,
    time: Res<Time>,
) {
    for (entity, mut cooldown) in query.iter_mut() {
        cooldown.cooldown.tick(time.delta());

        if cooldown.cooldown.just_finished() {
            commands.entity(entity).remove::<DamageCooldown>();
        }
    }
}

fn despawn_dead_entities(mut commands: Commands, query: Query<(Entity, &Health)>) {
    for (entity, health) in query.iter() {
        if health.amount == 0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn create_health_bars(
    mut commands: Commands,
    query: Query<Entity, Added<HealthBar>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let health_color = Color::YELLOW;
    let missing_color = Color::BLACK;
    for entity in query.iter() {
        commands.entity(entity).with_children(|children| {
            // Current health
            children.spawn((
                HealthBarDisplay,
                MaterialMesh2dBundle {
                    mesh: meshes
                        .add(shape::Quad::new(Vec2::new(12.0, 2.0)).into())
                        .into(),
                    material: materials.add(ColorMaterial::from(health_color)),
                    transform: Transform::from_translation(Vec3::new(0., 10.0, 101.0)),
                    ..Default::default()
                },
            ));
            // Missing health
            children.spawn(MaterialMesh2dBundle {
                mesh: meshes
                    .add(shape::Quad::new(Vec2::new(12.0, 2.0)).into())
                    .into(),
                material: materials.add(ColorMaterial::from(missing_color)),
                transform: Transform::from_translation(Vec3::new(0., 10.0, 100.0)),
                ..Default::default()
            });
        });
    }
}

fn update_health_bars(
    health_bars: Query<&Health, With<HealthBar>>,
    mut current_health: Query<(&Parent, &mut Transform), With<HealthBarDisplay>>,
) {
    for (parent, mut transform) in current_health.iter_mut() {
        let entity = parent.get();
        let Ok(health) = health_bars.get(entity) else {
            warn!("Health bar display where there is no health component");
            continue;
        };
        let percent = health.amount as f32 / health.total as f32;
        // Shift the bar's center leftwards as it shrinks
        transform.translation.x = (1.0 - percent) * -6.;
        transform.scale.x = percent;
    }
}
