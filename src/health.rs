use bevy::prelude::*;

use crate::{
    collision::Collider,
    ghost::Ghost,
    player::{Dagger, Player},
    schedule::InGame,
};

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                take_damage::<Player, Ghost>,
                take_damage::<Ghost, Dagger>,
                despawn_dead_entities,
            )
                .chain()
                .in_set(InGame::ProcessCombat),
        );
    }
}

const DAMAGE_COOLDOWN: f32 = 0.25;

#[derive(Component, Debug)]
pub struct Health {
    pub amount: u32,
    pub cooldown: Option<Timer>,
}

impl Health {
    pub fn new(amount: u32) -> Self {
        Self {
            amount,
            cooldown: None,
        }
    }
}

fn take_damage<T: Component, E: Component>(
    mut query: Query<(&mut Health, &Collider), With<T>>,
    enemies: Query<&E, With<Collider>>,
    time: Res<Time>,
) {
    for (mut health, collider) in query.iter_mut() {
        if let Some(cooldown) = health.cooldown.as_mut() {
            cooldown.tick(time.delta());
        }
        if collider.collisions.iter().any(|e| enemies.contains(*e)) {
            match health.cooldown.as_mut() {
                Some(cooldown) if cooldown.finished() => {
                    cooldown.reset();
                }
                None => {
                    health
                        .cooldown
                        .replace(Timer::from_seconds(DAMAGE_COOLDOWN, TimerMode::Once));
                }
                _ => {
                    continue;
                }
            }
            // None, Some + timer is finished
            health.amount = health.amount.saturating_sub(1);
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
