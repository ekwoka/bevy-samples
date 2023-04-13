use bevy::prelude::{ Vec2, Vec3, App, Plugin };

use crate::RADIAN_MAX;

mod constants;
pub mod boid_avoidance;
pub mod spawn_boids;

pub struct Boids;

impl Plugin for Boids {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_boids::spawn_boids).add_system(boid_avoidance::boid_avoidance);
    }
}

fn normalize_angle(angle: f32) -> f32 {
    ((angle + RADIAN_MAX * 1.5) % RADIAN_MAX) - RADIAN_MAX / 2.0
}

fn diff_angles(angle1: f32, angle2: f32) -> f32 {
    normalize_angle(angle1 - angle2)
}

fn relative_angle_between(base: Vec3, target: Vec3) -> f32 {
    let relative_position = target - base;
    relative_position.truncate().angle_between(Vec2::Y)
}
