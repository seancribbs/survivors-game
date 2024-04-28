use bevy::prelude::*;

use crate::{asset_loader::Fonts, schedule::AppState};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        // 1. setup the menu
        // 2. remove the menu items on exit
        // 3. handle user input
        app.add_systems(OnEnter(AppState::Menu), setup_menu)
            .add_systems(OnExit(AppState::Menu), cleanup_menu)
            .add_systems(
                Update,
                (handle_menu_input, blink_text)
                    .run_if(in_state(AppState::Menu).or_else(in_state(AppState::GameOver))),
            )
            .add_systems(OnEnter(AppState::GameOver), setup_end_screen)
            .add_systems(OnExit(AppState::GameOver), cleanup_end_screen);
    }
}

#[derive(Component, Debug, Default, Clone, Copy)]
pub struct Menu;

#[derive(Component, Debug, Default, Clone, Copy)]
pub struct GameOver;

#[derive(Component, Debug, Clone)]
pub struct Blink(Timer);

impl Default for Blink {
    fn default() -> Self {
        Self(Timer::from_seconds(0.5, TimerMode::Repeating))
    }
}

fn setup_menu(mut commands: Commands, font_assets: Res<Fonts>) {
    commands
        .spawn((
            Menu,
            NodeBundle {
                style: Style {
                    // center button
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(40.0),
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Bevy Survivors",
                TextStyle {
                    font: font_assets.press_start.clone(),
                    font_size: 40.0,
                    color: Color::WHITE,
                },
            ));
            parent.spawn((
                Blink::default(),
                TextBundle::from_section(
                    "PRESS ENTER TO PLAY",
                    TextStyle {
                        font: font_assets.press_start.clone(),
                        font_size: 18.0,
                        color: Color::YELLOW,
                    },
                ),
            ));
        });
}

fn cleanup_menu(mut commands: Commands, query: Query<Entity, With<Menu>>) {
    if let Ok(menu) = query.get_single() {
        commands.entity(menu).despawn_recursive();
    }
}

fn handle_menu_input(
    state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
    input: Res<Input<KeyCode>>,
) {
    for key in input.get_just_released() {
        if *key == KeyCode::Return {
            match state.get() {
                AppState::Menu => next_state.set(AppState::InGame),
                AppState::GameOver => next_state.set(AppState::Menu),
                _ => (),
            }
        }
    }
}

fn blink_text(mut query: Query<(&mut Visibility, &mut Blink)>, time: Res<Time>) {
    for (mut visibility, mut blink) in query.iter_mut() {
        if blink.0.tick(time.delta()).just_finished() {
            *visibility = match *visibility {
                Visibility::Inherited | Visibility::Visible => Visibility::Hidden,
                Visibility::Hidden => Visibility::Visible,
            };
        }
    }
}

fn setup_end_screen(mut commands: Commands, font_assets: Res<Fonts>) {
    commands
        .spawn((
            GameOver,
            NodeBundle {
                style: Style {
                    // center button
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(100.0),
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "GAME OVER",
                TextStyle {
                    font: font_assets.press_start.clone(),
                    font_size: 50.0,
                    color: Color::MAROON,
                },
            ));
            parent.spawn((
                Blink::default(),
                TextBundle::from_section(
                    "PRESS ENTER TO CONTINUE",
                    TextStyle {
                        font: font_assets.press_start.clone(),
                        font_size: 18.0,
                        color: Color::YELLOW,
                    },
                ),
            ));
        });
}

fn cleanup_end_screen(mut commands: Commands, query: Query<Entity, With<GameOver>>) {
    if let Ok(menu) = query.get_single() {
        commands.entity(menu).despawn_recursive();
    }
}
