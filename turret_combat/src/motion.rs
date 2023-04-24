use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct MotionCharacteristics {
    pub vector: Vec3,
    pub mass: f32,
    pub direction: f32,
}

pub const RADIAN_MAX: f32 = (360.0 * std::f32::consts::PI) / 180.0;
pub struct MotionPlugin;

impl Plugin for MotionPlugin {
    fn build(&self, _app: &mut App) {}
}
