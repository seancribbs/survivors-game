use bevy::prelude::*;

use crate::{collision::Collider, player::Player, schedule::InGame};

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, take_damage.in_set(InGame::ProcessCombat));
    }
}

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

fn take_damage(mut query: Query<(&mut Health, &Collider), With<Player>>, time: Res<Time>) {
    let Ok((mut health, collider)) = query.get_single_mut() else {
        return;
    };
    if let Some(cooldown) = health.cooldown.as_mut() {
        cooldown.tick(time.delta());
    }
    if !collider.collisions.is_empty() {
        match health.cooldown.as_mut() {
            Some(cooldown) if cooldown.finished() => {
                cooldown.reset();
            }
            None => {
                health
                    .cooldown
                    .replace(Timer::from_seconds(0.25, TimerMode::Once));
            }
            _ => {
                return;
            }
        }
        // None, Some + timer is finished
        health.amount = health.amount.saturating_sub(1);
        info!("Player got hit, health is now {}", health.amount);
    }
}
