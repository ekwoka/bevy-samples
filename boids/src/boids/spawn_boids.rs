use super::constants;
use bevy::ecs::system::Commands;
use bevy::prelude::{AssetServer, Query, Res, Transform, With};
use bevy::sprite::SpriteBundle;
use bevy::window::{PrimaryWindow, Window};
use rand::prelude::random;

use crate::{Vector, RADIAN_MAX};

pub fn spawn_boids(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    println!("vision arc is {} radians", constants::BOID_VISION_ARC);
    println!("vision distance is {}", constants::BOID_VISION_DISTANCE);
    println!("max radians is {}", RADIAN_MAX);
    let window = window_query.get_single().unwrap();
    for _ in 0..constants::BOID_COUNT {
        let size = rand::random::<f32>() * 0.1 + 0.2;
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(
                    random::<f32>() * window.width(),
                    random::<f32>() * window.height(),
                    0.0,
                )
                .with_scale([size, size, 1.0].into()),
                texture: asset_server.load("sprites/kenney_simple-space/PNG/Retina/ship_B.png"),
                ..Default::default()
            },
            Vector {
                direction: rand::random::<f32>() * 360.0,
                velocity: random::<f32>() * 50.0 + 75.0,
            },
        ));
    }
}
