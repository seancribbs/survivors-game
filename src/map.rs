use bevy::prelude::*;

use crate::{asset_loader::SpriteAssets, collision::Collider};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, add_outer_walls)
            .insert_resource(Map {
                dimensions: IVec2::new(100, 75),
                tile_size: 16,
            });
    }
}

#[derive(Resource, Debug)]
pub struct Map {
    pub dimensions: IVec2,
    pub tile_size: u32,
}

impl Map {
    pub fn half_extent(&self) -> IVec2 {
        self.dimensions / 2
    }
}

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct Wall;

#[inline]
pub fn grid_to_world(position: IVec2) -> Vec3 {
    (position * 16).extend(-1).as_vec3()
}

#[derive(Bundle, Debug, Default)]
pub struct WallBundle {
    pub wall: Wall,
    pub collider: Collider,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: VisibilityBundle,
}

// 250x250 (* 16) -- -125..125
fn add_outer_walls(mut commands: Commands, sprites: Res<SpriteAssets>, map: Res<Map>) {
    let half = map.half_extent();
    // Top wall
    commands
        .spawn(WallBundle {
            collider: Collider::with_size_and_offset(
                Vec2::new(grid_to_world(map.dimensions).x, 16.0),
                Vec2::new(0., 4.),
            ),
            transform: Transform::from_translation(grid_to_world(IVec2::new(0, half.y))),
            ..Default::default()
        })
        .with_children(|children| {
            for x in -half.x..=half.x {
                children.spawn(SpriteBundle {
                    texture: sprites.wall.hwall_face.clone(),
                    transform: Transform::from_translation(grid_to_world(IVec2::new(x, 0))),
                    ..Default::default()
                });
            }
        });

    // Bottom wall
    commands
        .spawn(WallBundle {
            collider: Collider::with_size_and_offset(
                Vec2::new(grid_to_world(map.dimensions).x, 12.0),
                Vec2::new(0., -4.),
            ),
            transform: Transform::from_translation(grid_to_world(IVec2::new(0, -half.y))),
            ..Default::default()
        })
        .with_children(|children| {
            for x in -half.x..=half.x {
                children.spawn(SpriteBundle {
                    texture: sprites.wall.hwall_top_mid.clone(),
                    transform: Transform::from_translation(grid_to_world(IVec2::new(x, 0))),
                    ..Default::default()
                });
            }
        });

    // Left wall
    commands
        .spawn(WallBundle {
            collider: Collider::with_size_and_offset(
                Vec2::new(6., grid_to_world(map.dimensions).y),
                Vec2::new(-2., 0.),
            ),
            transform: Transform::from_translation(grid_to_world(IVec2::new(-half.x, 0))),
            ..Default::default()
        })
        .with_children(|children| {
            for y in -half.y..=half.y {
                children.spawn(SpriteBundle {
                    texture: sprites.wall.vwall_left.clone(),
                    transform: Transform::from_translation(grid_to_world(IVec2::new(0, y))),
                    ..Default::default()
                });
            }
        });

    // Right wall
    commands
        .spawn(WallBundle {
            collider: Collider::with_size_and_offset(
                Vec2::new(6., grid_to_world(map.dimensions).y),
                Vec2::new(2., 0.),
            ),
            transform: Transform::from_translation(grid_to_world(IVec2::new(half.x, 0))),
            ..Default::default()
        })
        .with_children(|children| {
            for y in -half.y..=half.y {
                children.spawn(SpriteBundle {
                    texture: sprites.wall.vwall_right.clone(),
                    transform: Transform::from_translation(grid_to_world(IVec2::new(0, y))),
                    ..Default::default()
                });
            }
        });
}
