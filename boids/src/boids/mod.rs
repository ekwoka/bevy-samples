use bevy::prelude::{ Vec2, Vec3, App, Plugin };

use crate::RADIAN_MAX;

mod constants;
mod boid_avoidance;
mod spawn_boids;
mod boid_jitter;
mod boid_convergence;
mod boid_contest;

pub struct Boids;

impl Plugin for Boids {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_boids::spawn_boids)
            .add_system(boid_avoidance::boid_avoidance)
            .add_system(boid_jitter::boid_jitter)
            .add_system(boid_convergence::boid_convergence)
            .add_system(boid_contest::boid_contest);
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
