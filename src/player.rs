use crate::{
    asset_loader::SpriteAssets,
    movement::{MovementBundle, Velocity},
};
use bevy::prelude::*;

const PLAYER_SPEED: f32 = 50.;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_player)
            .add_systems(Update, player_movement);
    }
}

#[derive(Component, Debug)]
pub struct Player;

fn spawn_player(mut commands: Commands, sprite_assets: Res<SpriteAssets>) {
    commands.spawn((
        Player,
        SpriteBundle {
            texture: sprite_assets.knight.clone(),
            transform: Transform::from_xyz(0., 0., 0.),
            ..Default::default()
        },
        MovementBundle {
            velocity: Velocity::new(0., 0.),
        },
    ));
}

fn player_movement(mut query: Query<&mut Velocity, With<Player>>, input: Res<Input<KeyCode>>) {
    let mut direction = Vec3::ZERO;
    for key in input.get_pressed() {
        match key {
            KeyCode::W | KeyCode::Up => {
                direction += Vec3::Y;
            }
            KeyCode::A | KeyCode::Left => {
                direction += Vec3::NEG_X;
            }
            KeyCode::S | KeyCode::Down => {
                direction += Vec3::NEG_Y;
            }
            KeyCode::D | KeyCode::Right => {
                direction += Vec3::X;
            }
            _ => {}
        }
    }
    let mut velocity = query.single_mut();
    velocity.change_direction_speed(direction, PLAYER_SPEED);
}
