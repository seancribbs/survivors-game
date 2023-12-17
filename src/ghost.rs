use bevy::prelude::*;
use rand::Rng;

use crate::asset_loader::SpriteAssets;
use crate::movement::{MovementBundle, Velocity};
use crate::player::Player;

const GHOST_SPEED: f32 = 30.;
const SPAWN_INTERVAL: f32 = 0.5;

#[derive(Component, Debug)]
pub struct Ghost;

pub struct GhostPlugin;

#[derive(Resource, Debug)]
struct SpawnTimer(Timer);

impl Default for SpawnTimer {
    fn default() -> Self {
        SpawnTimer(Timer::from_seconds(SPAWN_INTERVAL, TimerMode::Repeating))
    }
}

impl Plugin for GhostPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer::default())
            .add_systems(Update, spawn_ghost)
            .add_systems(Update, chase_player);
    }
}

fn spawn_ghost(
    mut commands: Commands,
    sprite_assets: Res<SpriteAssets>,
    mut timer: ResMut<SpawnTimer>,
    time: Res<Time>,
) {
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(-1.0f32..1.0f32);
        let y = rng.gen_range(-1.0f32..1.0f32);
        let spawn_spot =
            Vec3::new(x, y, 0.).try_normalize().unwrap_or(Vec3::X) * Vec3::new(410.0, 310.0, 0.);
        let direction = Vec3::ZERO - spawn_spot;

        commands.spawn((
            Ghost,
            SpriteBundle {
                texture: sprite_assets.ghost.clone(),
                transform: Transform::from_translation(spawn_spot),
                ..Default::default()
            },
            MovementBundle {
                velocity: Velocity::from_direction_speed(direction, GHOST_SPEED),
            },
        ));
    }
}

fn chase_player(
    mut ghosts: Query<(&mut Velocity, &Transform), With<Ghost>>,
    player: Query<&Transform, With<Player>>,
) {
    let player_transform = player.single();
    for (mut velocity, transform) in ghosts.iter_mut() {
        let direction = player_transform.translation - transform.translation;
        velocity.change_direction_speed(direction, GHOST_SPEED);
    }
}
