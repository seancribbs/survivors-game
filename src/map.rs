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

#[derive(Component, Debug, Clone, Copy)]
pub struct Wall;

#[inline]
pub fn grid_to_world(position: IVec2) -> Vec3 {
    (position * 16).extend(-1).as_vec3()
}

// 250x250 (* 16) -- -125..125
fn add_outer_walls(mut commands: Commands, sprites: Res<SpriteAssets>, map: Res<Map>) {
    let half = map.half_extent();
    // horizontal walls
    for x in -half.x..=half.x {
        for (y, top_of_wall) in [(-half.y, true), (half.y, false)] {
            let texture = if top_of_wall {
                &sprites.wall.hwall_top_mid
            } else {
                &sprites.wall.hwall_face
            }
            .clone();

            commands.spawn((
                Wall,
                if top_of_wall {
                    Collider::with_size_and_offset(Vec2::new(16., 12.), Vec2::new(0., -4.))
                } else {
                    Collider::with_size_and_offset(Vec2::new(16., 8.), Vec2::new(0., 4.))
                },
                SpriteBundle {
                    texture,
                    transform: Transform::from_translation(grid_to_world(IVec2::new(x, y))),
                    ..Default::default()
                },
            ));
        }
    }
    // vertical walls
    for y in -half.y..=half.y {
        for (x, left_wall) in [(-half.x, true), (half.x, false)] {
            let texture = if left_wall {
                &sprites.wall.vwall_left
            } else {
                &sprites.wall.vwall_right
            }
            .clone();

            let offset = if left_wall {
                Vec2::new(-2., 0.)
            } else {
                Vec2::new(2., 0.)
            };
            commands.spawn((
                Wall,
                Collider::with_size_and_offset(Vec2::new(6., 16.), offset),
                SpriteBundle {
                    texture,
                    transform: Transform::from_translation(grid_to_world(IVec2::new(x, y))),
                    ..Default::default()
                },
            ));
        }
    }
}
