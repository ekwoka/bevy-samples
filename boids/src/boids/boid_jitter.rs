use bevy::{
    prelude::{Query, Res, Transform, Without},
    time::Time,
};
use rand::Rng;

use crate::{Player, Vector, RADIAN_MAX};

pub fn boid_jitter(
    mut boid_query: Query<(&mut Vector, &Transform), Without<Player>>,
    time: Res<Time>,
) {
    for (mut vector, _) in boid_query
        .iter_mut()
        .filter(|(_, transform)| !transform.translation.x.is_nan())
    {
        vector.direction += rand::thread_rng().gen_range(-5.0..=5.0) * time.delta_seconds();
        vector.direction %= RADIAN_MAX;
    }
}
