use bevy::prelude::*;

use crate::{map::grid_to_world, map::Map, player::Player, schedule::InGame};

const CAMERA_SCALE: f32 = 0.75;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, camera_follows_player.in_set(InGame::EntityUpdates));
    }
}

fn spawn_camera(mut commands: Commands) {
    let mut bundle = Camera2dBundle::default();
    bundle.projection.scale = CAMERA_SCALE;
    commands.spawn(bundle);
}

fn camera_follows_player(
    player: Query<&Transform, With<Player>>,
    mut camera: Query<(&mut Transform, &OrthographicProjection), (With<Camera>, Without<Player>)>,
    map: Res<Map>,
) {
    if let Ok(player_transform) = player.get_single() {
        let (mut camera_transform, projection) = camera.single_mut();

        let bottom_left = grid_to_world(-(map.dimensions / 2));
        let top_right = grid_to_world(map.dimensions / 2);

        let min_camera_position = bottom_left - projection.area.min.extend(0.);
        let max_camera_position = top_right - projection.area.max.extend(0.);
        camera_transform.translation = player_transform
            .translation
            .clamp(min_camera_position, max_camera_position);
    }
}
