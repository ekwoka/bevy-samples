use bevy::{ prelude::{ Query, Transform, Without, Res }, time::Time };
use rand::Rng;

use crate::{ Vector, Player, RADIAN_MAX };

pub fn boid_jitter(
    mut boid_query: Query<(&mut Vector, &Transform), Without<Player>>,
    time: Res<Time>
) {
    for (mut vector, transform) in boid_query.iter_mut() {
        if transform.translation.x.is_nan() {
            continue;
        }
        vector.direction += rand::thread_rng().gen_range(-5.0..=5.0) * time.delta_seconds();
        vector.direction %= RADIAN_MAX;
    }
}
