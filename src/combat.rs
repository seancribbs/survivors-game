use bevy::prelude::*;

use crate::{collision::Collider, ghost::Ghost, player::Player, schedule::InGame};

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, apply_knockback.in_set(InGame::EntityUpdates))
            .add_systems(
                Update,
                knockback_collisions::<Player, Ghost>.in_set(InGame::ProcessCombat),
            );
    }
}

const KNOCK_BACK_DISTANCE: f32 = 32.0;
const KNOCK_BACK_DURATION: f32 = 0.1;

#[derive(Component, Debug)]
pub struct KnockBack {
    pub displacement: Vec3,
    pub duration: Timer,
}

fn apply_knockback(
    mut query: Query<(Entity, &mut KnockBack, &mut Transform)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (entity, mut knockback, mut transform) in query.iter_mut() {
        knockback.duration.tick(time.delta());
        transform.translation += knockback.displacement
            * (time.delta_seconds() / knockback.duration.duration().as_secs_f32());
        if knockback.duration.finished() {
            commands.entity(entity).remove::<KnockBack>();
        }
    }
}

fn knockback_collisions<T: Component, C: Component>(
    mut commands: Commands,
    collidee: Query<(Entity, &Collider, &Transform, &T), Without<KnockBack>>,
    collided: Query<(&Transform, &C), With<Collider>>,
) {
    for (entity, collider, transform, _) in collidee.iter() {
        let mut direction: Vec3 = Vec3::ZERO;
        for collided_entity in collider.collisions.iter() {
            if let Ok((collided_transform, _)) = collided.get(*collided_entity) {
                direction += transform.translation - collided_transform.translation;
            }
        }
        commands.entity(entity).insert(KnockBack {
            displacement: direction.normalize_or_zero() * KNOCK_BACK_DISTANCE,
            duration: Timer::from_seconds(KNOCK_BACK_DURATION, TimerMode::Once),
        });
    }
}
