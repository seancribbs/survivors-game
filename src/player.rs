use crate::{
    asset_loader::SpriteAssets,
    collision::{Collider, CollisionDamage},
    health::Health,
    movement::{MovementBundle, Velocity},
    schedule::InGame,
};
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

const PLAYER_SPEED: f32 = 50.;
const PLAYER_SIZE: Vec2 = Vec2::splat(16.);
const PLAYER_STARTING_HEALTH: u32 = 30;
const PLAYER_ATTACK_COOLDOWN: f32 = 1.0;
const PLAYER_DAMAGE_COOLDOWN: f32 = 0.25;
const DAGGER_SPEED: f32 = 100.0;
const DAGGER_SPAWN_DISTANCE: f32 = 16.0;
const DAGGER_DAMAGE: u32 = 5;
const DAGGER_HEALTH: u32 = 1;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<PlayerBundle>("player")
            .add_systems(Update, throw_weapon.in_set(InGame::ProcessCombat))
            .add_systems(Update, player_movement.in_set(InGame::UserInput));
    }
}

#[derive(Component, Debug, Default)]
pub struct Player;

#[derive(Component, Debug)]
pub struct Weapon(Timer);

#[derive(Component, Debug)]
pub struct Dagger;

#[derive(Bundle, LdtkEntity)]
pub struct PlayerBundle {
    player: Player,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
    health: Health,
    collider: Collider,
    weapon: Weapon,
    movement: MovementBundle,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            player: Player,
            sprite_sheet_bundle: Default::default(),
            health: Health::with_damage_cooldown(PLAYER_STARTING_HEALTH, PLAYER_DAMAGE_COOLDOWN),
            collider: Collider::new(PLAYER_SIZE),
            weapon: Weapon(Timer::from_seconds(
                PLAYER_ATTACK_COOLDOWN,
                TimerMode::Repeating,
            )),
            movement: Default::default(),
        }
    }
}

fn player_movement(
    mut query: Query<(&mut Velocity, &mut Transform), With<Player>>,
    input: Res<Input<KeyCode>>,
) {
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
    let Ok((mut velocity, mut transform)) = query.get_single_mut() else {
        return;
    };
    transform.translation.z = 100.0;
    velocity.change_direction_speed(direction, PLAYER_SPEED);
}

fn throw_weapon(
    mut query: Query<(&mut Weapon, &Transform), With<Player>>,
    time: Res<Time>,
    mut commands: Commands,
    sprite_assets: Res<SpriteAssets>,
) {
    let Ok((mut weapon, player_transform)) = query.get_single_mut() else {
        return;
    };
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
