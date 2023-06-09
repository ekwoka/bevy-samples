use bevy::app::App;
use bevy::ecs::system::Commands;
use bevy::prelude::*;
use bevy::window::{PrimaryWindow, Window};
use bevy::DefaultPlugins;

mod motion;
mod player;
mod render;
use motion::MotionPlugin;
use player::PlayerPlugin;
use render::update_sprites;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(System)
        .run();
}

fn setup_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 1000.0),
        ..Default::default()
    });
}
pub struct System;

impl Plugin for System {
    fn build(&self, app: &mut App) {
        app.add_plugin(PlayerPlugin)
            .add_plugin(MotionPlugin)
            .add_startup_system(setup_camera)
            .add_system(wrap_screen_edge)
            .add_system(update_sprites);
    }
}

fn wrap_screen_edge(
    mut transform_query: Query<&mut Transform>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    let left_bound = window.width() * -0.05;
    let right_bound = window.width() * 1.05;
    let top_bound = window.height() * 1.05;
    let bottom_bound = window.height() * -0.05;
    let height_offset = window.height() * 0.025;
    let width_offset = window.width() * 0.025;
    for mut transform in transform_query.iter_mut() {
        if transform.translation.x < left_bound {
            transform.translation.x = right_bound - width_offset;
        } else if transform.translation.x > right_bound {
            transform.translation.x = left_bound + width_offset;
        }
        if transform.translation.y > top_bound {
            transform.translation.y = bottom_bound + height_offset;
        } else if transform.translation.y < bottom_bound {
            transform.translation.y = top_bound - height_offset;
        }
    }
}
