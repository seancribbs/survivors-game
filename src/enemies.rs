use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
// use bevy_ecs_ldtk::utils::{grid_coords_to_translation, translation_to_grid_coords};
// use rand::Rng;

// use crate::asset_loader::SpriteAssets;
use crate::collision::{Collider, CollisionDamage};
use crate::health::Health;
// use crate::levels::SpawnLocations;
use crate::movement::{MovementBundle, Velocity};
use crate::player::Player;
use crate::schedule::InGame;

// const GHOST_SPEED: f32 = 30.;
const SPAWN_INTERVAL: f32 = 0.5;
// const GHOST_SIZE: Vec2 = Vec2::splat(16.);
// const GHOST_DAMAGE: u32 = 5;
// const GHOST_HEALTH: u32 = 10;

#[derive(Component, Debug, Default)]
pub struct Ghost;

#[derive(Bundle, LdtkEntity, Default)]
pub struct GhostBundle {
    kind: Ghost,
    #[ldtk_entity]
    enemy: EnemyBundle,
}

#[derive(Component, Debug, Default)]
pub struct Cyclops;

#[derive(Bundle, LdtkEntity, Default)]
pub struct CyclopsBundle {
    kind: Cyclops,
    #[ldtk_entity]
    enemy: EnemyBundle,
}

#[derive(Component, Debug, Default)]
pub struct Crab;

#[derive(Bundle, LdtkEntity, Default)]
pub struct CrabBundle {
    kind: Crab,
    #[ldtk_entity]
    enemy: EnemyBundle,
}

#[derive(Component, Debug, Default)]
pub struct Bat;

#[derive(Bundle, LdtkEntity, Default)]
pub struct BatBundle {
    kind: Bat,
    #[ldtk_entity]
    enemy: EnemyBundle,
}

#[derive(Component, Debug, Default)]
pub struct Spider;

#[derive(Bundle, LdtkEntity, Default)]
pub struct SpiderBundle {
    kind: Spider,
    #[ldtk_entity]
    enemy: EnemyBundle,
}

#[derive(Component, Debug, Default)]
pub struct Enemy;

#[derive(Bundle, LdtkEntity, Default)]
pub struct EnemyBundle {
    enemy: Enemy,
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
        app.insert_resource(SpawnTimer::default())
            .register_ldtk_entity::<GhostBundle>("ghost")
            // .add_systems(Update, spawn_ghost)
            .register_ldtk_entity::<CyclopsBundle>("cyclops")
            .register_ldtk_entity::<CrabBundle>("crab")
            .register_ldtk_entity::<BatBundle>("bat")
            .register_ldtk_entity::<SpiderBundle>("spider")
            .add_systems(Update, chase_player.in_set(InGame::EntityUpdates));
    }
}

// #[allow(clippy::too_many_arguments)]
// fn spawn_ghost(
//     mut commands: Commands,
//     sprite_assets: Res<SpriteAssets>,
//     mut timer: ResMut<SpawnTimer>,
//     time: Res<Time>,
//     level_selection: Res<LevelSelection>,
//     // level_query: Query<(Entity, &LevelIid)>,
//     ldtk_projects: Query<&Handle<LdtkProject>>,
//     ldtk_project_assets: Res<Assets<LdtkProject>>,
//     player: Query<&Transform, With<Player>>,
//     spawn_locations: Res<SpawnLocations>,
// ) {
//     timer.0.tick(time.delta());
//     if timer.0.just_finished() {
//         let ldtk_project = ldtk_project_assets
//             .get(ldtk_projects.single())
//             .expect("Project should be loaded if level has spawned");

//         let level = ldtk_project
//             .as_standalone()
//             .find_loaded_level_by_level_selection(&level_selection)
//             .expect("Selected level should exist in LDTK project");

//         let Some(level_spawn_locations) = spawn_locations.for_level(level.iid()) else {
//             return;
//         };

//         let LayerInstance {
//             c_hei: height,
//             c_wid: width,
//             grid_size,
//             ..
//         } = level.layer_instances()[0];
//         let grid_size = IVec2::splat(grid_size);
//         // 1. Find the position of the player, convert to GridCoords
//         let origin = if let Ok(player_transform) = player.get_single() {
//             player_transform.translation.truncate()
//         } else {
//             Vec2::ZERO
//         };
//         let origin_grid_coords = translation_to_grid_coords(origin, grid_size);
//         // 2. Pick an unoccupied tile to spawn the ghost in, giving up after 5 attempts
//         for _ in 0..5 {
//             let mut rng = rand::thread_rng();
//             let angle = rng.gen_range(0f32..(std::f32::consts::TAU));
//             let unit_vector = Vec2::from_angle(angle);
//             let vector_on_square = if unit_vector.x.abs() < unit_vector.y.abs() {
//                 unit_vector / unit_vector.y.abs()
//             } else {
//                 unit_vector / unit_vector.x.abs()
//             };
//             let spawn_spot = vector_on_square * grid_size.as_vec2() * 16.0;
//             let spawn_coords =
//                 translation_to_grid_coords(spawn_spot, grid_size) + origin_grid_coords;
//             let spawn_coords = GridCoords::new(
//                 spawn_coords.x.clamp(0, width),
//                 spawn_coords.y.clamp(0, height),
//             );

//             if level_spawn_locations.contains(&spawn_coords) {
//                 let spawn_spot = grid_coords_to_translation(spawn_coords, grid_size);
//                 let direction = Vec3::ZERO - spawn_spot.extend(0.);
//                 commands.spawn(GhostBundle::new(
//                     &sprite_assets,
//                     spawn_spot.extend(0.),
//                     direction,
//                 ));
//                 return;
//             }
//         }
//     }
// }

fn chase_player(
    mut enemies: Query<(&mut Velocity, &Transform), With<Enemy>>,
    player: Query<&Transform, With<Player>>,
) {
    let Ok(player_transform) = player.get_single() else {
        return;
    };
    for (mut velocity, transform) in enemies.iter_mut() {
        let direction = player_transform.translation - transform.translation;
        velocity.change_direction(direction);
    }
}
