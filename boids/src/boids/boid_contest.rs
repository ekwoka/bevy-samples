use bevy::{ prelude::{ Transform, Without, Query, Res, Vec3 }, time::Time };

use crate::{ Vector, Player, RADIAN_MAX };

use super::{ diff_angles, constants, relative_angle_between };

pub fn boid_contest(
    mut boids: Query<(&mut Vector, &Transform), Without<Player>>,
    other_boids: Query<&Transform, Without<Player>>,
    time: Res<Time>
) {
    for (mut vector, transform) in boids.iter_mut() {
        if transform.translation.x.is_nan() {
            continue;
        }
        let mut target: Vec3 = Vec3::default();
        let mut count = 0;
        for other_transform in other_boids.iter() {
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
                if relative_angle.abs() < constants::BOID_VISION_ARC / 2.0 {
                    count += 1;
                    target += other_transform.translation - transform.translation;
                }
            }
        }
        if count == 0 {
            continue;
        }
        target =
            target / Vec3::from([count as f32, count as f32, count as f32]) + transform.translation;
        let absolute_angle = relative_angle_between(transform.translation, target);
        let relative_angle = diff_angles(absolute_angle, vector.direction);
        vector.direction += 20.0 * relative_angle * time.delta_seconds();
        vector.direction %= RADIAN_MAX;
    }
}
