use bevy::{
    prelude::{Query, Res, Transform, Vec3, Without},
    time::Time,
};

use crate::{Player, Vector, RADIAN_MAX};

use super::{constants, diff_angles, relative_angle_between};

pub fn boid_contest(
    mut boids: Query<(&mut Vector, &Transform), Without<Player>>,
    other_boids: Query<&Transform, Without<Player>>,
    time: Res<Time>,
) {
    for (mut vector, transform) in boids
        .iter_mut()
        .filter(|(_, transform)| !transform.translation.x.is_nan())
    {
        let mut target: Vec3 = Vec3::default();
        let mut count = 0;
        for other_transform in other_boids.iter().filter(|other_transform| {
            transform != *other_transform && !other_transform.translation.x.is_nan()
        }) {
            let distance = other_transform.translation.distance(transform.translation);
            if distance < constants::BOID_VISION_DISTANCE {
                let absolute_angle =
                    relative_angle_between(transform.translation, other_transform.translation);
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
        target = target / count as f32 + transform.translation;
        let absolute_angle = relative_angle_between(transform.translation, target);
        let relative_angle = diff_angles(absolute_angle, vector.direction);
        vector.direction += 20.0 * relative_angle * time.delta_seconds();
        vector.direction %= RADIAN_MAX;
    }
}
