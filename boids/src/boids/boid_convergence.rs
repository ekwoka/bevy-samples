use bevy::{ prelude::{ Query, Transform, Without, Res }, time::Time };

use crate::{ Vector, Player, RADIAN_MAX };

use super::{ constants, diff_angles };

pub fn boid_convergence(
    mut boids: Query<(&mut Vector, &Transform), Without<Player>>,
    other_boids: Query<&Transform, Without<Player>>,
    time: Res<Time>
) {
    for (mut vector, transform) in boids.iter_mut() {
        if transform.translation.x.is_nan() {
            continue;
        }
        let mut direction = 0.0;
        for other_transform in other_boids.iter() {
            if transform == other_transform || other_transform.translation.x.is_nan() {
                continue;
            }
            let distance = other_transform.translation.distance(transform.translation);
            if distance < constants::BOID_VISION_DISTANCE {
                let other_direction = other_transform.rotation.z;
                let relative_angle = diff_angles(vector.direction, other_direction);
                direction +=
                    relative_angle.signum() *
                    2.0 *
                    (1.0 - distance / constants::BOID_VISION_DISTANCE);
            }
        }
        vector.direction += direction * time.delta_seconds();
        vector.direction %= RADIAN_MAX;
    }
}
