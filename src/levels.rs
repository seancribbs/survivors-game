use std::collections::{HashMap, HashSet};

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::collision::Collider;

pub struct LevelsPlugin;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct Wall;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct WallTile;

#[derive(Bundle, Debug, Default, LdtkIntCell)]
pub struct WallBundle {
    pub wall: WallTile,
}

#[derive(Resource, Default, Debug)]
pub struct SpawnLocations(HashMap<String, HashSet<GridCoords>>);

impl SpawnLocations {
    pub fn for_level(&self, iid: &str) -> Option<&HashSet<GridCoords>> {
        self.0.get(iid)
    }
}

#[derive(Resource, Default, Debug)]
pub struct ActiveSpawnList(Vec<(String, i32)>);

impl ActiveSpawnList {
    pub fn pop_spawn(&mut self) -> Option<&str> {
        for (name, count) in self.0.iter_mut() {
            if *count > 0 {
                *count -= 1;
                return Some(&*name);
            }
        }
        None
    }
}

impl Plugin for LevelsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LevelSelection::index(0))
            .init_resource::<SpawnLocations>()
            .init_resource::<ActiveSpawnList>()
            .register_ldtk_int_cell::<WallBundle>(1)
            .add_systems(Startup, load_levels)
            .add_systems(
                Update,
                (
                    add_wall_colliders,
                    preload_spawn_spots,
                    populate_active_spawns,
                ),
            );
    }
}

fn load_levels(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("levels/basic.ldtk"),
        ..Default::default()
    });
}

fn preload_spawn_spots(
    mut spawn_locations: ResMut<SpawnLocations>,
    ground_query: Query<(&GridCoords, &Parent, &TileMetadata), Added<TileMetadata>>,
    parent_query: Query<&Parent, Without<TileMetadata>>,
    level_query: Query<(Entity, &LevelIid)>,
) {
    if ground_query.is_empty() {
        return;
    }
    let mut level_to_spawn_locations: HashMap<Entity, HashSet<GridCoords>> = HashMap::new();
    for (grid_coords, parent, metadata) in ground_query.iter() {
        if metadata.data == "Ground" {
            if let Ok(grandparent) = parent_query.get(parent.get()) {
                level_to_spawn_locations
                    .entry(grandparent.get())
                    .or_default()
                    .insert(*grid_coords);
            }
        }
    }
    spawn_locations.0 = level_query
        .iter()
        .map(|(entity, iid)| {
            (
                iid.get().clone(),
                level_to_spawn_locations.remove(&entity).unwrap_or_default(),
            )
        })
        .collect();
}

fn populate_active_spawns(
    mut level_events: EventReader<LevelEvent>,
    mut active_spawns: ResMut<ActiveSpawnList>,
    ldtk_projects: Query<&Handle<LdtkProject>>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
) {
    for level_event in level_events.read() {
        if let LevelEvent::Spawned(level_iid) = level_event {
            info!("Setting active spawn list for level {}", level_iid.as_str());
            let ldtk_project = ldtk_project_assets
                .get(ldtk_projects.single())
                .expect("Project should be loaded if level has spawned");

            let level = ldtk_project
                .as_standalone()
                .get_loaded_level_by_iid(&level_iid.as_str().to_owned())
                .expect("Loaded level should exist");

            let names: Vec<String> = level
                .field_instances()
                .iter()
                .find_map(|fi| {
                    if fi.identifier == "enemy_types" {
                        if let FieldValue::Strings(strs) = &fi.value {
                            return Some(strs.clone().into_iter().flatten().collect());
                        }
                    }
                    None
                })
                .expect("Didn't find enemy_types field on level");
            let counts: Vec<i32> = level
                .field_instances()
                .iter()
                .find_map(|fi| {
                    if fi.identifier == "enemy_counts" {
                        if let FieldValue::Ints(ints) = &fi.value {
                            return Some(ints.clone().into_iter().flatten().collect());
                        }
                    }
                    None
                })
                .expect("Didn't find enemy_counts field on level");
            info!(
                "Got spawns {names:?} => {counts:?} for level {}",
                level_iid.as_str()
            );
            active_spawns.0 = names.into_iter().zip(counts.into_iter()).collect();
        }
    }
}

fn add_wall_colliders(
    mut commands: Commands,
    wall_query: Query<(&GridCoords, &Parent, Option<&TileEnumTags>), Added<WallTile>>,
    parent_query: Query<&Parent, Without<WallTile>>,
    level_query: Query<(Entity, &LevelIid)>,
    ldtk_projects: Query<&Handle<LdtkProject>>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
) {
    /// Represents a wide wall that is 1 tile tall
    /// Used to spawn wall collisions
    #[derive(Clone, Eq, PartialEq, Debug, Default, Hash)]
    struct Plate {
        left: i32,
        right: i32,
    }

    /// A simple rectangle type representing a wall of any size
    struct Rect {
        left: i32,
        right: i32,
        top: i32,
        bottom: i32,
    }

    // Consider where the walls are
    // storing them as GridCoords in a HashSet for quick, easy lookup
    //
    // The key of this map will be the entity of the level the wall belongs to.
    // This has two consequences in the resulting collision entities:
    // 1. it forces the walls to be split along level boundaries
    // 2. it lets us easily add the collision entities as children of the appropriate level entity
    let mut level_to_wall_locations: HashMap<Entity, HashMap<GridCoords, HashSet<String>>> =
        HashMap::new();

    wall_query.for_each(|(&grid_coords, parent, tile_enum_tags)| {
        // An intgrid tile's direct parent will be a layer entity, not the level entity
        // To get the level entity, you need the tile's grandparent.
        // This is where parent_query comes in.
        if let Ok(grandparent) = parent_query.get(parent.get()) {
            let tags: HashSet<String> = tile_enum_tags
                .map(|t| t.tags.clone().into_iter().collect())
                .unwrap_or_default();
            level_to_wall_locations
                .entry(grandparent.get())
                .or_default()
                .insert(grid_coords, tags);
        }
    });

    if !wall_query.is_empty() {
        level_query.for_each(|(level_entity, level_iid)| {
            if let Some(level_walls) = level_to_wall_locations.get(&level_entity) {
                let ldtk_project = ldtk_project_assets
                    .get(ldtk_projects.single())
                    .expect("Project should be loaded if level has spawned");

                let level = ldtk_project
                    .as_standalone()
                    .get_loaded_level_by_iid(&level_iid.to_string())
                    .expect("Spawned level should exist in LDtk project");

                let LayerInstance {
                    c_wid: width,
                    c_hei: height,
                    grid_size,
                    ..
                } = level.layer_instances()[0];

                // combine wall tiles into flat "plates" in each individual row
                let mut plate_stack: Vec<Vec<(Plate, HashSet<String>)>> = Vec::new();

                for y in 0..height {
                    let mut row_plates: Vec<(Plate, HashSet<String>)> = Vec::new();
                    let mut plate_start = None;

                    // + 1 to the width so the algorithm "terminates" plates that touch the right edge
                    for x in 0..width + 1 {
                        match (plate_start, level_walls.get(&GridCoords { x, y })) {
                            // End of a wall segment
                            (Some((s, tags)), None) => {
                                row_plates.push((
                                    Plate {
                                        left: s,
                                        right: x - 1,
                                    },
                                    tags,
                                ));
                                plate_start = None;
                            }
                            // Start of a wall segment
                            (None, Some(tags)) => plate_start = Some((x, tags.clone())),
                            // Continuation of a wall segment, collecting all the tile tags
                            (Some((s, existing_tags)), Some(tags)) => {
                                plate_start =
                                    Some((s, existing_tags.union(tags).cloned().collect()))
                            }
                            // Still not a wall
                            (None, None) => plate_start = None,
                        }
                    }

                    plate_stack.push(row_plates);
                }

                // combine "plates" into rectangles across multiple rows
                let mut rect_builder: HashMap<Plate, (Rect, HashSet<String>)> = HashMap::new();
                let mut prev_row: Vec<(Plate, HashSet<String>)> = Vec::new();
                let mut wall_rects: Vec<(Rect, HashSet<String>)> = Vec::new();

                // an extra empty row so the algorithm "finishes" the rects that touch the top edge
                plate_stack.push(Vec::new());

                for (y, current_row) in plate_stack.into_iter().enumerate() {
                    for (prev_plate, _) in &prev_row {
                        if !current_row
                            .iter()
                            .any(|(current_plate, _)| prev_plate == current_plate)
                        {
                            // remove the finished rect so that the same plate in the future starts a new rect
                            if let Some(rect) = rect_builder.remove(prev_plate) {
                                wall_rects.push(rect);
                            }
                        }
                    }
                    for (plate, tags) in &current_row {
                        rect_builder
                            .entry(plate.clone())
                            .and_modify(|(e, t)| {
                                e.top += 1;
                                t.extend(tags.clone());
                            })
                            .or_insert((
                                Rect {
                                    bottom: y as i32,
                                    top: y as i32,
                                    left: plate.left,
                                    right: plate.right,
                                },
                                tags.clone(),
                            ));
                    }
                    prev_row = current_row;
                }

                commands.entity(level_entity).with_children(|level| {
                    // Spawn colliders for every rectangle..
                    // Making the collider a child of the level serves two purposes:
                    // 1. Adjusts the transforms to be relative to the level for free
                    // 2. the colliders will be despawned automatically when levels unload
                    for (wall_rect, tags) in wall_rects {
                        let mut offset = Vec2::ZERO;
                        let mut width = ((wall_rect.right - wall_rect.left + 1) * grid_size) as f32;
                        let mut height =
                            ((wall_rect.top - wall_rect.bottom + 1) * grid_size) as f32;
                        for tag in tags {
                            match tag.as_ref() {
                                "left" => {
                                    width -= 10.;
                                    offset.x -= 2.;
                                }
                                "right" => {
                                    width -= 10.;
                                    offset.x += 2.;
                                }
                                "top" => {
                                    height -= 4.;
                                    offset.y += 4.;
                                }
                                "bottom" => {
                                    height -= 4.;
                                    offset.y -= 4.;
                                }
                                _ => (),
                            }
                        }
                        level.spawn((
                            Wall,
                            Collider::with_size_and_offset(Vec2::new(width, height), offset),
                            SpatialBundle {
                                transform: Transform::from_xyz(
                                    (wall_rect.left + wall_rect.right + 1) as f32
                                        * grid_size as f32
                                        / 2.,
                                    (wall_rect.bottom + wall_rect.top + 1) as f32
                                        * grid_size as f32
                                        / 2.,
                                    0.,
                                ),
                                ..Default::default()
                            },
                        ));
                    }
                });
            }
        });
    }
}
