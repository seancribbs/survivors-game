use bevy::ecs::query::WorldQuery;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_ecs_ldtk::utils::{grid_coords_to_translation, translation_to_grid_coords};
use rand::Rng as _;
// use bevy_ecs_ldtk::utils::{grid_coords_to_translation, translation_to_grid_coords};
// use rand::Rng;

// use crate::asset_loader::SpriteAssets;
use crate::collision::{Collider, CollisionDamage};
use crate::health::Health;
use crate::levels::{ActiveSpawnList, SpawnLocations};
// use crate::levels::SpawnLocations;
use crate::movement::{Facing, MovementBundle, Velocity};
use crate::player::Player;
use crate::schedule::InGame;

const SPAWN_INTERVAL: f32 = 0.5;
const ENEMY_ENTITIES: [&str; 5] = ["ghost", "cyclops", "crab", "bat", "spider"];

#[derive(Component, Debug, Default)]
pub struct Enemy;

#[derive(Component, Debug, Default)]
pub struct EnemyPrototype(String);

impl From<&EntityInstance> for EnemyPrototype {
    fn from(value: &EntityInstance) -> Self {
        Self(value.identifier.clone())
    }
}

#[derive(Bundle, LdtkEntity, Default)]
pub struct EnemyPrototypeBundle {
    #[from_entity_instance]
    enemy: EnemyPrototype,
    #[sprite_sheet_bundle]
    sprite: SpriteSheetBundle,
    #[from_entity_instance]
    collider: Collider,
    #[from_entity_instance]
    damage: CollisionDamage,
    #[from_entity_instance]
    health: Health,
    #[from_entity_instance]
    movement: MovementBundle,
}

#[derive(Bundle, Default)]
pub struct EnemyBundle {
    enemy: Enemy,
    sprite: SpriteSheetBundle,
    collider: Collider,
    damage: CollisionDamage,
    health: Health,
    movement: MovementBundle,
}

pub struct EnemiesPlugin;

#[derive(Resource, Debug)]
struct SpawnTimer(Timer);

impl Default for SpawnTimer {
    fn default() -> Self {
        SpawnTimer(Timer::from_seconds(SPAWN_INTERVAL, TimerMode::Repeating))
    }
}

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        let app = app.insert_resource(SpawnTimer::default());
        for enemy in ENEMY_ENTITIES {
            app.register_ldtk_entity::<EnemyPrototypeBundle>(enemy);
        }
        app.add_systems(Update, cleanup_enemy_prototypes)
            .add_systems(
                Update,
                (chase_player, spawn_enemy).in_set(InGame::EntityUpdates),
            );
    }
}

fn cleanup_enemy_prototypes(query: Query<Entity, Added<EnemyPrototype>>, mut commands: Commands) {
    for entity in query.iter() {
        commands.entity(entity).remove::<SpatialBundle>();
    }
}

#[derive(WorldQuery)]
struct EnemyPrototypeQuery {
    prototype: &'static EnemyPrototype,
    collider: &'static Collider,
    collision_damage: &'static CollisionDamage,
    health: &'static Health,
    sprite: &'static TextureAtlasSprite,
    atlas: &'static Handle<TextureAtlas>,
    velocity: &'static Velocity,
}

#[allow(clippy::too_many_arguments)]
fn spawn_enemy(
    mut commands: Commands,
    mut timer: ResMut<SpawnTimer>,
    mut spawns: ResMut<ActiveSpawnList>,
    time: Res<Time>,
    prototypes: Query<EnemyPrototypeQuery>,
    level_selection: Res<LevelSelection>,
    ldtk_projects: Query<&Handle<LdtkProject>>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
    player: Query<&Transform, With<Player>>,
    spawn_locations: Res<SpawnLocations>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let Some(spawn_id) = spawns.pop_spawn() else {
            return;
        };
        let Some(spot) = pick_spawn_location(
            level_selection,
            ldtk_projects,
            ldtk_project_assets,
            player,
            spawn_locations,
        ) else {
            info!("Couldn't spawn an enemy after 5 attempts");
            return;
        };
        let transform = Transform::from_translation(spot.extend(100.));
        for prototype in prototypes.iter() {
            if prototype.prototype.0 == spawn_id {
                commands.spawn(EnemyBundle {
                    enemy: Enemy,
                    sprite: SpriteSheetBundle {
                        sprite: prototype.sprite.clone(),
                        texture_atlas: prototype.atlas.clone(),
                        transform,
                        ..Default::default()
                    },
                    collider: prototype.collider.clone(),
                    damage: prototype.collision_damage.clone(),
                    health: prototype.health.clone(),
                    movement: MovementBundle {
                        velocity: prototype.velocity.clone(),
                        ..Default::default()
                    },
                });
            }
        }
    }
}

fn pick_spawn_location(
    level_selection: Res<LevelSelection>,
    // level_query: Query<(Entity, &LevelIid)>,
    ldtk_projects: Query<&Handle<LdtkProject>>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
    player: Query<&Transform, With<Player>>,
    spawn_locations: Res<SpawnLocations>,
) -> Option<Vec2> {
    let ldtk_project = ldtk_project_assets
        .get(ldtk_projects.single())
        .expect("Project should be loaded if level has spawned");

    let level = ldtk_project
        .as_standalone()
        .find_loaded_level_by_level_selection(&level_selection)
        .expect("Selected level should exist in LDTK project");

    let level_spawn_locations = spawn_locations.for_level(level.iid())?;

    let LayerInstance {
        c_hei: height,
        c_wid: width,
        grid_size,
        ..
    } = level.layer_instances()[0];
    let grid_size = IVec2::splat(grid_size);
    // 1. Find the position of the player, convert to GridCoords
    let origin = if let Ok(player_transform) = player.get_single() {
        player_transform.translation.truncate()
    } else {
        Vec2::ZERO
    };
    let origin_grid_coords = translation_to_grid_coords(origin, grid_size);
    // 2. Pick an unoccupied tile to spawn the ghost in, giving up after 5 attempts
    for _ in 0..5 {
        let mut rng = rand::thread_rng();
        let angle = rng.gen_range(0f32..(std::f32::consts::TAU));
        let unit_vector = Vec2::from_angle(angle);
        let vector_on_square = if unit_vector.x.abs() < unit_vector.y.abs() {
            unit_vector / unit_vector.y.abs()
        } else {
            unit_vector / unit_vector.x.abs()
        };
        let spawn_spot = vector_on_square * grid_size.as_vec2() * 16.0;
        let spawn_coords = translation_to_grid_coords(spawn_spot, grid_size) + origin_grid_coords;
        let spawn_coords = GridCoords::new(
            spawn_coords.x.clamp(0, width),
            spawn_coords.y.clamp(0, height),
        );

        if level_spawn_locations.contains(&spawn_coords) {
            let spawn_spot = grid_coords_to_translation(spawn_coords, grid_size);
            return Some(spawn_spot);
        }
    }
    None
}

fn chase_player(
    mut enemies: Query<(&mut Velocity, &Transform, &mut Facing), With<Enemy>>,
    player: Query<&Transform, With<Player>>,
) {
    let Ok(player_transform) = player.get_single() else {
        return;
    };
    for (mut velocity, transform, mut facing) in enemies.iter_mut() {
        let direction = player_transform.translation - transform.translation;
        facing.value = direction.normalize_or_zero();
        velocity.change_direction(direction);
    }
}
