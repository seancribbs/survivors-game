use std::ops::AddAssign;

use bevy::{prelude::*, utils::HashMap};

use crate::{collision::CollisionEvent, ghost::Ghost, player::Player, schedule::InGame};

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
    mut events: EventReader<CollisionEvent>,
    receivers: Query<&Transform, (With<T>, Without<KnockBack>)>,
    pushers: Query<&Transform, With<C>>,
) {
    let mut knockbacks: HashMap<Entity, Vec3> = HashMap::new();
    for event in events.read() {
        let Ok(receiver_transform) = receivers.get(event.entity) else {
            continue;
        };

        let Ok(pusher_transform) = pushers.get(event.collided_with) else {
            continue;
        };

        let push_direction = receiver_transform.translation - pusher_transform.translation;
        knockbacks
            .entry(event.entity)
            .and_modify(|direction| direction.add_assign(push_direction))
            .or_insert(push_direction);
    }

    for (entity, direction) in knockbacks.into_iter() {
        commands.entity(entity).insert(KnockBack {
            displacement: direction.normalize_or_zero() * KNOCK_BACK_DISTANCE,
            duration: Timer::from_seconds(KNOCK_BACK_DURATION, TimerMode::Once),
        });
    }
}
