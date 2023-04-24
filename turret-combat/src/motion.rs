use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct MotionCharacteristics {
    pub vector: Vec3,
    pub mass: u32,
    pub direction: f32,
}

pub const RADIAN_MAX: f32 = (360.0 * std::f32::consts::PI) / 180.0;
pub struct MotionPlugin;

impl Plugin for MotionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(vacuum_drag);
    }
}

fn vacuum_drag(mut motion_query: Query<&mut MotionCharacteristics>, time: Res<Time>) {
    let mut motion = motion_query.single_mut();
    let mut vector = motion.vector;
    let drag = 0.4;
    let drag_mod = 1.0 - drag * time.delta_seconds();
    vector *= drag_mod;
    motion.vector = vector;
}
