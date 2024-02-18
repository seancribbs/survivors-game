use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

mod asset_loader;
mod camera;
mod collision;
mod combat;
mod ghost;
mod health;
mod levels;
mod movement;
mod player;
mod schedule;

fn main() {
    App::new()
        // Bevy built-ins
        .insert_resource(ClearColor(Color::rgb(0.1, 0., 0.15)))
        .add_plugins(DefaultPlugins.set(
            WindowPlugin {
                primary_window: Some(Window {
                    title: "Bevy Survivors".into(),
                    resolution: (800., 600.).into(),
                    ..Default::default()
                }),
                ..Default::default()
            }).set(ImagePlugin::default_nearest())
        )
        // Our plugins
        .add_plugins(LdtkPlugin)
        .add_plugins(schedule::SchedulePlugin)
        .add_plugins(camera::CameraPlugin)
        .add_plugins(asset_loader::AssetLoaderPlugin)
        .add_plugins(levels::LevelsPlugin)
        .add_plugins(movement::MovementPlugin)
        .add_plugins(ghost::GhostPlugin)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(collision::CollisionPlugin)
        .add_plugins(health::HealthPlugin)
        .add_plugins(combat::CombatPlugin)
        // .add_systems(PostUpdate, print_position)
        .run();
}

#[allow(dead_code)]
fn print_position(query: Query<(Entity, &Transform)>) {
    // Log the entity ID and position of each entity with a `Position` component
    for (entity, transform) in query.iter() {
        info!("Entity {entity:?} is at position {transform:?}");
    }
}
