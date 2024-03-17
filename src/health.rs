use bevy::prelude::*;
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
        .add_systems(Update, tick_damage_cooldown.in_set(InGame::EntityUpdates));
    }
}

#[derive(Component, Debug, Default)]
pub struct Health {
    pub amount: u32,
    pub cooldown: Option<f32>,
}

impl From<&EntityInstance> for Health {
    fn from(value: &EntityInstance) -> Self {
        if let Ok(v) = value.get_int_field("health") {
            Self {
                amount: *v as u32,
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
            cooldown: None,
        }
    }

    pub fn with_damage_cooldown(amount: u32, cooldown: f32) -> Self {
        Self {
            amount,
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
