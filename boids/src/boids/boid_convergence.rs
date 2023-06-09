use bevy::{
    prelude::{Query, Res, Transform, Without},
    time::Time,
};

use crate::{Player, Vector, RADIAN_MAX};

use super::{constants, diff_angles, relative_angle_between};

pub fn boid_convergence(
    mut boids: Query<(&mut Vector, &Transform), Without<Player>>,
    other_boids: Query<&Transform, Without<Player>>,
    time: Res<Time>,
) {
    for (mut vector, transform) in boids
        .iter_mut()
        .filter(|(_, transform)| !transform.translation.x.is_nan())
    {
        let direction = other_boids
            .iter()
            .filter(|other_transform| {
                !other_transform.translation.x.is_nan() && transform != *other_transform
            })
            .fold(0.0, |mut direction, other_transform| {
                let distance = other_transform.translation.distance(transform.translation);
                if distance < constants::BOID_VISION_DISTANCE {
                    let absolute_angle =
                        relative_angle_between(transform.translation, other_transform.translation);
                    let relative_angle = diff_angles(absolute_angle, vector.direction);
                    if relative_angle.abs() < constants::BOID_VISION_ARC / 2.0 {
                        let other_direction = other_transform.rotation.z * RADIAN_MAX;
                        let relative_angle = diff_angles(vector.direction, other_direction);
                        direction += relative_angle.signum()
                            * 2.0
                            * (1.0 - distance / constants::BOID_VISION_DISTANCE);
                    }
                };
                direction
            });
        vector.direction += direction * time.delta_seconds();
        vector.direction %= RADIAN_MAX;
    }
}
