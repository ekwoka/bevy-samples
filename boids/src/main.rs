use bevy::app::App;
use bevy::ecs::component::Component;
use bevy::ecs::system::Commands;
use bevy::prelude::{
    AssetServer, Camera2dBundle, Plugin, Quat, Query, Res, Resource, Transform, With,
};

use bevy::time::{Time, Timer};
use bevy::window::{PrimaryWindow, Window};
use bevy::DefaultPlugins;

mod boids;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(System)
        .run();
}

#[derive(Resource)]
struct CheckTimer(Timer);

#[derive(Debug, Component)]
pub struct Player {}

#[derive(Debug, Component)]
pub struct Vector {
    direction: f32,
    velocity: f32,
}

fn setup_env(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    _asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 1000.0),
        ..Default::default()
    });
}

pub struct System;

impl Plugin for System {
    fn build(&self, app: &mut App) {
        app.add_plugin(boids::Boids)
            .add_startup_system(setup_env)
            .add_system(vector_motion)
            .add_system(wrap_screen_edge);
    }
}

pub const RADIAN_MAX: f32 = (360.0 * std::f32::consts::PI) / 180.0;

fn vector_motion(time: Res<Time>, mut transformation_vectors: Query<(&mut Transform, &Vector)>) {
    for (mut transform, vector) in transformation_vectors.iter_mut() {
        transform.translation.y += vector.velocity * time.delta_seconds() * vector.direction.cos();
        transform.translation.x += vector.velocity * time.delta_seconds() * vector.direction.sin();
        transform.rotation = Quat::from_rotation_z(-vector.direction).normalize();
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
    for mut transform in transform_query.iter_mut() {
        if transform.translation.x < left_bound {
            transform.translation.x = right_bound;
        } else if transform.translation.x > right_bound {
            transform.translation.x = left_bound;
        }
        if transform.translation.y > top_bound {
            transform.translation.y = bottom_bound;
        } else if transform.translation.y < bottom_bound {
            transform.translation.y = top_bound;
        }
    }
}
