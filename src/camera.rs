use bevy::prelude::*;

const CAMERA_SCALE: f32 = 0.75;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    let mut bundle = Camera2dBundle::default();
    bundle.projection.scale = CAMERA_SCALE;
    commands.spawn(bundle);
}
