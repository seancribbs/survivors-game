use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::{player::Player, schedule::InGame};

const CAMERA_SCALE: f32 = 0.75;

pub struct CameraPlugin;

#[derive(Bundle, Default, LdtkEntity)]
pub struct EntityCameraBundle {
    camera: Camera2dBundle,
}

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<EntityCameraBundle>("camera")
            .add_systems(Update, camera_follows_player.in_set(InGame::EntityUpdates));
    }
}

fn camera_follows_player(
    player: Query<&Transform, With<Player>>,
    mut camera: Query<(&mut Transform, &OrthographicProjection), (With<Camera>, Without<Player>)>,
    // level_query: Query<(&Transform, &LevelIid), (Without<OrthographicProjection>, Without<Player>)>,
    // level_selection: Res<LevelSelection>,
    // ldtk_projects: Query<&Handle<LdtkProject>>,
    // ldtk_project_assets: Res<Assets<LdtkProject>>,
) {
    let Ok((mut camera_transform, _projection)) = camera.get_single_mut() else {
        return;
    };

    let Ok(player_transform) = player.get_single() else {
        return;
    };

    camera_transform.translation = player_transform.translation;
    // if let Ok(player_transform) = player.get_single() {
    //     // for (level_transform, level_iid) in &level_query {
    //     //     let ldtk_project = ldtk_project_assets
    //     //         .get(ldtk_projects.single())
    //     //         .expect("Project should be loaded if level has spawned");

    //     //     let level = ldtk_project
    //     //         .get_raw_level_by_iid(&level_iid.to_string())
    //     //         .expect("Spawned level should exist in LDtk project");

    //     //     if level_selection.is_match(&LevelIndices::default(), level) {
    //     //         let top_right = (IVec2::new(level.px_wid, level.px_hei).as_vec2() / 2.).extend(0.);
    //     //         let bottom_left = -top_right;

    //     //         let min_camera_position =
    //     //             bottom_left - projection.area.min.extend(0.) + level_transform.translation;
    //     //         let max_camera_position =
    //     //             top_right - projection.area.max.extend(0.) + level_transform.translation;
    //     //         camera_transform.translation = player_transform
    //     //             .translation
    //     //             .clamp(min_camera_position, max_camera_position);
    //     //     }
    //     // }
    // }
}
