use crate::{
    asset_loader::SpriteAssets,
    collision::{Collider, CollisionDamage},
    health::Health,
    movement::{MovementBundle, Velocity},
    schedule::InGame,
};
use bevy::prelude::*;

const PLAYER_SPEED: f32 = 50.;
const PLAYER_SIZE: Vec2 = Vec2::splat(16.);
const PLAYER_STARTING_HEALTH: u32 = 30;
const PLAYER_ATTACK_COOLDOWN: f32 = 1.0;
const PLAYER_DAMAGE_COOLDOWN: f32 = 0.25;
const DAGGER_SPEED: f32 = 25.0;
const DAGGER_SPAWN_DISTANCE: f32 = 16.0;
const DAGGER_DAMAGE: u32 = 5;
const DAGGER_HEALTH: u32 = 1;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_player)
            .add_systems(Update, throw_weapon.in_set(InGame::ProcessCombat))
            .add_systems(Update, player_movement.in_set(InGame::UserInput));
    }
}

#[derive(Component, Debug)]
pub struct Player;

#[derive(Component, Debug)]
pub struct Weapon(Timer);

#[derive(Component, Debug)]
pub struct Dagger;

fn spawn_player(mut commands: Commands, sprite_assets: Res<SpriteAssets>) {
    commands.spawn((
        Player,
        SpriteBundle {
            texture: sprite_assets.knight.clone(),
            transform: Transform::from_xyz(0., 0., 0.),
            ..Default::default()
        },
        Health::with_damage_cooldown(PLAYER_STARTING_HEALTH, PLAYER_DAMAGE_COOLDOWN),
        Collider::new(PLAYER_SIZE),
        Weapon(Timer::from_seconds(
            PLAYER_ATTACK_COOLDOWN,
            TimerMode::Repeating,
        )),
        MovementBundle {
            velocity: Velocity::new(0., 0.),
        },
    ));
}

fn player_movement(mut query: Query<&mut Velocity, With<Player>>, input: Res<Input<KeyCode>>) {
    let mut direction = Vec3::ZERO;
    for key in input.get_pressed() {
        match key {
            KeyCode::W | KeyCode::Up => {
                direction += Vec3::Y;
            }
            KeyCode::A | KeyCode::Left => {
                direction += Vec3::NEG_X;
            }
            KeyCode::S | KeyCode::Down => {
                direction += Vec3::NEG_Y;
            }
            KeyCode::D | KeyCode::Right => {
                direction += Vec3::X;
            }
            _ => {}
        }
    }
    // NOTE: If the player has died/been despawned from losing all its health, this will panic.
    // We need to solve this by changing game states or guarding this access.
    let mut velocity = query.single_mut();
    velocity.change_direction_speed(direction, PLAYER_SPEED);
}

fn throw_weapon(
    mut query: Query<(&mut Weapon, &Transform), With<Player>>,
    time: Res<Time>,
    mut commands: Commands,
    sprite_assets: Res<SpriteAssets>,
) {
    let (mut weapon, player_transform) = query.single_mut();
    weapon.0.tick(time.delta());
    if weapon.0.just_finished() {
        for (i, direction) in [Vec3::Y, Vec3::NEG_X, Vec3::NEG_Y, Vec3::X]
            .into_iter()
            .enumerate()
        {
            let mut transform = *player_transform;
            transform.translation += direction * DAGGER_SPAWN_DISTANCE;
            let rotation = Quat::from_axis_angle(Vec3::Z, (i as f32) * std::f32::consts::FRAC_PI_2);
            transform.rotate(rotation);
            commands.spawn((
                Dagger,
                SpriteBundle {
                    texture: sprite_assets.dagger.clone(),
                    transform,
                    ..Default::default()
                },
                Collider::new(Vec2::new(8., 13.)),
                Health::new(DAGGER_HEALTH),
                CollisionDamage::new(DAGGER_DAMAGE),
                MovementBundle {
                    velocity: Velocity::from_direction_speed(direction, DAGGER_SPEED),
                },
            ));
        }
    }
}
