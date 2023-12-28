use bevy::prelude::*;

pub struct SchedulePlugin;

impl Plugin for SchedulePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                InGame::ProcessCombat,
                InGame::UserInput,
                InGame::EntityUpdates,
                InGame::CollisionDetection,
            )
                .chain(),
        )
        .add_systems(
            Update,
            apply_deferred
                .before(InGame::UserInput)
                .after(InGame::ProcessCombat),
        );
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum InGame {
    ProcessCombat,
    UserInput,
    EntityUpdates,
    CollisionDetection,
}
