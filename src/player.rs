use crate::{
    asset_loader::SpriteAssets,
    collision::{Collider, CollisionDamage},
    health::{Health, HealthBar},
    movement::{Facing, MovementBundle, Velocity},
    schedule::{AppState, InGame},
};
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use rand::random;

const PLAYER_SPEED: f32 = 50.;
const PLAYER_SIZE: Vec2 = Vec2::splat(16.);
const PLAYER_STARTING_HEALTH: u32 = 30;
const PLAYER_ATTACK_COOLDOWN: f32 = 1.0;
const PLAYER_DAMAGE_COOLDOWN: f32 = 0.25;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<PlayerBundle>("player")
            .add_systems(Update, throw_weapon.in_set(InGame::ProcessCombat))
            .add_systems(Update, player_movement.in_set(InGame::UserInput))
            .add_systems(Update, game_over.in_set(InGame::EntityUpdates))
        // .add_systems(OnExit(AppState::InGame), reset_player)
        ;
    }
}

#[derive(Component, Debug, Default)]
pub struct Player;

#[derive(Debug, Default, Clone, Copy)]
pub enum WeaponType {
    #[default]
    Dagger,
}

#[derive(Debug, Default, Clone, Copy)]
pub enum WeaponSpawnBehavior {
    #[default]
    FourDirections,
    Facing,
    Random,
}

#[derive(Debug, Clone)]
pub struct WeaponSpec {
    speed: f32,
    distance: f32,
    damage: u32,
    health: u32,
    behavior: WeaponSpawnBehavior,
    collider: Collider,
}

impl From<WeaponType> for WeaponSpec {
    fn from(value: WeaponType) -> Self {
        match value {
            WeaponType::Dagger => Self {
                speed: 100.0,
                distance: 16.0,
                damage: 5,
                health: 1,
                collider: Collider::new(Vec2::new(8., 13.)),
                behavior: WeaponSpawnBehavior::FourDirections,
            },
        }
    }
}

#[derive(Component, Debug)]
pub struct Weapon {
    kind: WeaponType,
    cooldown: Timer,
}

#[derive(Component, Debug)]
pub struct Projectile;

#[derive(Bundle, LdtkEntity)]
pub struct PlayerBundle {
    player: Player,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
    health: Health,
    collider: Collider,
    weapon: Weapon,
    movement: MovementBundle,
    health_bar: HealthBar,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            player: Player,
            sprite_sheet_bundle: Default::default(),
            health: Health::with_damage_cooldown(PLAYER_STARTING_HEALTH, PLAYER_DAMAGE_COOLDOWN),
            collider: Collider::new(PLAYER_SIZE),
            weapon: Weapon {
                kind: Default::default(),
                cooldown: Timer::from_seconds(PLAYER_ATTACK_COOLDOWN, TimerMode::Repeating),
            },
            movement: MovementBundle {
                facing: Facing { value: Vec3::X },
                ..Default::default()
            },
            health_bar: HealthBar,
        }
    }
}

fn player_movement(
    mut query: Query<(&mut Velocity, &mut Transform, &mut Facing), With<Player>>,
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
    let Ok((mut velocity, mut transform, mut facing)) = query.get_single_mut() else {
        return;
    };
    if direction != Vec3::ZERO {
        facing.value = direction;
    }
    transform.translation.z = 100.0;
    velocity.change_direction_speed(direction, PLAYER_SPEED);
}

#[derive(Bundle)]
pub struct WeaponBundle {
    projectile: Projectile,
    spritesheet: SpriteSheetBundle,
    collider: Collider,
    health: Health,
    collision_damage: CollisionDamage,
    movement: MovementBundle,
}

fn throw_weapon(
    mut query: Query<(&mut Weapon, &Transform, &Facing), With<Player>>,
    time: Res<Time>,
    mut commands: Commands,
    sprite_assets: Res<SpriteAssets>,
) {
    let Ok((mut weapon, player_transform, facing)) = query.get_single_mut() else {
        return;
    };
    weapon.cooldown.tick(time.delta());
    if weapon.cooldown.just_finished() {
        let spec: WeaponSpec = weapon.kind.into();

        let spawns: Vec<(Transform, Velocity)> = match &spec.behavior {
            WeaponSpawnBehavior::FourDirections => [Vec3::Y, Vec3::NEG_X, Vec3::NEG_Y, Vec3::X]
                .into_iter()
                .enumerate()
                .map(|(i, direction)| {
                    let mut transform = *player_transform;
                    transform.translation += direction * spec.distance;
                    let rotation =
                        Quat::from_axis_angle(Vec3::Z, (i as f32) * std::f32::consts::FRAC_PI_2);
                    transform.rotate(rotation);
                    let velocity = Velocity::from_direction_speed(direction, spec.speed);
                    (transform, velocity)
                })
                .collect(),
            WeaponSpawnBehavior::Facing => {
                let mut transform = *player_transform;
                let direction = facing.value;
                transform.translation += direction * spec.distance;
                let rotation = Quat::from_axis_angle(
                    Vec3::Z,
                    if direction.y.signum() < 0. {
                        Vec3::NEG_X.angle_between(direction) + std::f32::consts::FRAC_PI_2
                    } else {
                        Vec3::X.angle_between(direction) - std::f32::consts::FRAC_PI_2
                    }, // TODO: What is the correct mathematical way to deal with this
                );
                transform.rotate(rotation);
                vec![(
                    transform,
                    Velocity::from_direction_speed(direction, spec.speed),
                )]
            }
            WeaponSpawnBehavior::Random => {
                let mut transform = *player_transform;
                let angle = random::<f32>() * std::f32::consts::TAU;
                let direction = Vec2::from_angle(angle).extend(0.);
                let rotation = Quat::from_axis_angle(
                    Vec3::Z,
                    if direction.y.signum() < 0. {
                        Vec3::NEG_X.angle_between(direction) + std::f32::consts::FRAC_PI_2
                    } else {
                        Vec3::X.angle_between(direction) - std::f32::consts::FRAC_PI_2
                    }, // TODO: What is the correct mathematical way to deal with this
                );
                transform.translation += direction * spec.distance;
                transform.rotate(rotation);
                vec![(
                    transform,
                    Velocity::from_direction_speed(direction, spec.speed),
                )]
            }
        };

        let sprite = match weapon.kind {
            WeaponType::Dagger => TextureAtlasSprite::new(103),
        };

        for (transform, velocity) in spawns {
            commands.spawn(WeaponBundle {
                spritesheet: SpriteSheetBundle {
                    texture_atlas: sprite_assets.tiles.clone(),
                    sprite: sprite.clone(),
                    transform,
                    ..Default::default()
                },
                movement: MovementBundle {
                    velocity,
                    ..Default::default()
                },
                projectile: Projectile,
                collider: spec.collider.clone(),
                health: Health::new(spec.health),
                collision_damage: CollisionDamage::new(spec.damage),
            });
        }
    }
}

fn game_over(mut next_state: ResMut<NextState<AppState>>, removed: RemovedComponents<Player>) {
    if !removed.is_empty() {
        next_state.set(AppState::GameOver);
    }
}
