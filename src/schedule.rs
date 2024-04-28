use bevy::prelude::*;

pub struct SchedulePlugin;

impl Plugin for SchedulePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<AppState>()
            .configure_sets(
                Update,
                (
                    InGame::ProcessCombat,
                    InGame::UserInput,
                    InGame::EntityUpdates,
                    InGame::CollisionDetection,
                )
                    .chain()
                    .run_if(in_state(AppState::InGame)),
            )
            .add_systems(
                Update,
                apply_deferred
                    .before(InGame::UserInput)
                    .after(InGame::ProcessCombat)
                    .run_if(in_state(AppState::InGame)),
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

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    #[default]
    Menu,
    InGame,
    GameOver,
}
