use bevy::prelude::*;

use crate::motion::MotionCharacteristics;

pub fn update_sprites(
    mut transform_query: Query<(&mut Transform, &MotionCharacteristics)>,
    time: Res<Time>,
) {
    for (mut transform, motion) in transform_query.iter_mut() {
        transform.rotation = Quat::from_rotation_z(motion.direction);
        transform.translation += motion.vector * time.delta_seconds();
    }
}
