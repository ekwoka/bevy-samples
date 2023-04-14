use bevy::{ prelude::{ Transform, Query, Without, Res }, time::Time };

use crate::{ RADIAN_MAX, Vector, Player };

use super::{ constants, diff_angles, relative_angle_between };

pub fn boid_avoidance(
    mut boid_query: Query<(&mut Vector, &Transform), Without<Player>>,
    other_query: Query<&Transform, Without<Player>>,
    time: Res<Time>
) {
    for (mut vector, transform) in boid_query.iter_mut() {
        if transform.translation.x.is_nan() {
            continue;
        }
        let mut direction = 0.0;
        for other_transform in other_query.iter() {
            if transform == other_transform || other_transform.translation.x.is_nan() {
                continue;
            }
            let distance = other_transform.translation.distance(transform.translation);
            if distance < constants::BOID_VISION_DISTANCE {
                let absolute_angle = relative_angle_between(
                    transform.translation,
                    other_transform.translation
                );
                let relative_angle = diff_angles(absolute_angle, vector.direction);
                if relative_angle.abs() < constants::BOID_VISION_ARC {
                    direction -=
                        (4.0 - relative_angle.abs() / constants::BOID_VISION_ARC) *
                        relative_angle.signum() *
                        4.0 *
                        (0.75 - distance / constants::BOID_VISION_DISTANCE);
                }
            }
        }
        vector.direction += direction * time.delta_seconds();
        vector.direction %= RADIAN_MAX;
    }
}
