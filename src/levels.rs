use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::collision::Collider;

pub const TILE_SIZE: i32 = 16;
// pub const TILE_DIMENSIONS: IVec2 = IVec2::splat(TILE_SIZE);

pub struct LevelsPlugin;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct Wall;

#[derive(Bundle, Debug, Default, LdtkIntCell)]
pub struct WallBundle {
    pub wall: Wall,
}

impl Plugin for LevelsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LevelSelection::index(0))
            .register_ldtk_int_cell::<WallBundle>(1)
            .add_systems(Startup, load_levels)
            .add_systems(Update, add_wall_colliders);
    }
}

fn load_levels(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("levels/basic.ldtk"),
        ..Default::default()
    });
}

fn add_wall_colliders(mut commands: Commands, query: Query<(Entity, &TileEnumTags), Added<Wall>>) {
    for (entity, enum_tags) in query.iter() {
        let mut entity_commands = commands.entity(entity);

        for tag in &enum_tags.tags {
            match tag.as_ref() {
                "left" => {
                    entity_commands.insert(Collider::with_size_and_offset(
                        Vec2::new(6., TILE_SIZE as f32),
                        Vec2::new(-2., 0.),
                    ));
                }
                "right" => {
                    entity_commands.insert(Collider::with_size_and_offset(
                        Vec2::new(6., TILE_SIZE as f32),
                        Vec2::new(2., 0.),
                    ));
                }
                "top" => {
                    entity_commands.insert(Collider::with_size_and_offset(
                        Vec2::new(TILE_SIZE as f32, 12.0),
                        Vec2::new(0., 4.),
                    ));
                }
                "bottom" => {
                    entity_commands.insert(Collider::with_size_and_offset(
                        Vec2::new(TILE_SIZE as f32, 12.0),
                        Vec2::new(0., -4.),
                    ));
                }
                _ => (),
            }
        }
    }
}
