use bevy::{
    prelude::{Query, Res, Transform, Without},
    time::Time,
};

use crate::{Player, Vector, RADIAN_MAX};

use super::{constants, diff_angles, relative_angle_between};

pub fn boid_avoidance(
    mut boid_query: Query<(&mut Vector, &Transform), Without<Player>>,
    other_query: Query<&Transform, Without<Player>>,
    time: Res<Time>,
) {
    for (mut vector, transform) in boid_query
        .iter_mut()
        .filter(|(_, transform)| !transform.translation.x.is_nan())
    {
        let direction = other_query
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
                    if relative_angle.abs() < constants::BOID_VISION_ARC {
                        direction -= (4.0 - relative_angle.abs() / constants::BOID_VISION_ARC)
                            * relative_angle.signum()
                            * 4.0
                            * (0.75 - distance / constants::BOID_VISION_DISTANCE);
                    }
                }
                direction
            });
        vector.direction += direction * time.delta_seconds();
        vector.direction %= RADIAN_MAX;
    }
}
