use bevy::DefaultPlugins;
use bevy::ecs::system::Commands;
use bevy::app::App;
use bevy::ecs::component::Component;
use bevy::prelude::{
    Query,
    With,
    Plugin,
    Res,
    ResMut,
    Resource,
    AssetServer,
    Transform,
    Camera2dBundle,
    Input,
    KeyCode,
    Quat,
    Without,
    Vec3,
};
use bevy::sprite::SpriteBundle;
use bevy::time::{ Time, Timer, TimerMode };
use bevy::window::{ Window, PrimaryWindow };
use rand::prelude::random;

fn main() {
    App::new().add_plugins(DefaultPlugins).add_plugin(Boids).run();
}

#[derive(Resource)]
struct CheckTimer(Timer);

#[derive(Debug)]
#[derive(Component)]
struct Player {}

#[derive(Debug)]
#[derive(Component)]
struct Vector {
    direction: f32,
    velocity: f32,
}

fn setup_env(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
) {
    let window = window_query.get_single().unwrap();
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(
                window.width() / 2.0,
                window.height() / 2.0,
                0.0
            ).with_scale([0.25, 0.25, 1.0].into()),
            texture: asset_server.load("sprites/kenney_simple-space/PNG/Retina/ship_J.png"),
            ..Default::default()
        },
        Player {},
        Vector {
            direction: 0.0,
            velocity: 0.0,
        },
    ));
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 1000.0),
        ..Default::default()
    });
}
const BOID_COUNT: u16 = 200;

fn spawn_boids(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
) {
    println!("vision arc is {} radians", BOID_VISION_ARC);
    println!("vision distance is {}", BOID_VISION_DISTANCE);
    println!("max radians is {}", RADIAN_MAX);
    let window = window_query.get_single().unwrap();
    for _ in 0..BOID_COUNT {
        let size = rand::random::<f32>() * 0.1 + 0.2;
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(
                    random::<f32>() * window.width(),
                    random::<f32>() * window.height(),
                    0.0
                ).with_scale([size, size, 1.0].into()),
                texture: asset_server.load("sprites/kenney_simple-space/PNG/Retina/ship_B.png"),
                ..Default::default()
            },
            Vector {
                direction: rand::random::<f32>() * 360.0,
                velocity: random::<f32>() * 50.0 + 75.0,
            },
        ));
    }
}

const BOID_VISION_ARC: f32 = 0.7 * std::f32::consts::PI;
const BOID_VISION_DISTANCE: f32 = 100.0;

fn boid_avoidance(
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
            if distance < BOID_VISION_DISTANCE {
                let absolute_angle = relative_angle_between(
                    transform.translation,
                    other_transform.translation
                );
                let relative_angle = diff_angles(absolute_angle, vector.direction);
                if relative_angle.abs() < BOID_VISION_ARC {
                    direction -=
                        (2.0 - relative_angle.abs() / BOID_VISION_ARC) *
                        relative_angle.signum() *
                        5.0 *
                        (1.0 - distance / BOID_VISION_DISTANCE);
                }
            }
        }
        vector.direction += direction * time.delta_seconds();
        vector.direction %= RADIAN_MAX;
    }
}

fn normalize_angle(angle: f32) -> f32 {
    ((angle + RADIAN_MAX * 1.5) % RADIAN_MAX) - RADIAN_MAX / 2.0
}

fn diff_angles(angle1: f32, angle2: f32) -> f32 {
    normalize_angle(angle1 - angle2)
}

fn relative_angle_between(base: Vec3, target: Vec3) -> f32 {
    let relative_position = target - base;
    relative_position.angle_between(Vec3::Y) * relative_position.x.signum()
}

fn check_player(
    time: Res<Time>,
    mut timer: ResMut<CheckTimer>,
    player: Query<(&Player, &Vector, &Transform)>,
    boid_query: Query<&Transform, Without<Player>>
) {
    if timer.0.tick(time.delta()).just_finished() {
        if let Ok((player, vector, transform)) = player.get_single() {
            println!("{:?} is alive!", player);
            println!("{:?}", vector);
            println!("{:?}", transform);
            let mut count = 0;
            for boid_transform in boid_query.iter() {
                if boid_transform.translation.x.is_nan() {
                    count += 1;
                }
            }
            println!("{} boids are dead", count);
        }
    }
}

pub struct Boids;

impl Plugin for Boids {
    fn build(&self, app: &mut App) {
        app.insert_resource(CheckTimer(Timer::from_seconds(5.0, TimerMode::Repeating)))
            .add_startup_system(setup_env)
            .add_startup_system(spawn_boids)
            .add_system(check_player)
            .add_system(player_control)
            .add_system(vector_motion)
            .add_system(wrap_screen_edge)
            .add_system(boid_avoidance);
    }
}

const PLAYER_ACCEL: f32 = 160.0;
const PLAYER_DECEL: f32 = 80.0;
const PLAYER_MAX_VELOCITY: f32 = 800.0;
const RADIAN_MAX: f32 = (360.0 * std::f32::consts::PI) / 180.0;

fn player_control(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Vector, With<Player>>,
    time: Res<Time>
) {
    if let Ok(mut vector) = player_query.get_single_mut() {
        let speed_adjustment = time.delta_seconds() * PLAYER_ACCEL;
        if keyboard_input.pressed(KeyCode::Up) {
            vector.velocity += speed_adjustment;
        } else if keyboard_input.pressed(KeyCode::Down) {
            vector.velocity -= speed_adjustment;
        } else {
            if vector.velocity > 0.0 {
                if vector.velocity < time.delta_seconds() * PLAYER_DECEL {
                    vector.velocity = 0.0;
                } else {
                    vector.velocity -= time.delta_seconds() * PLAYER_DECEL;
                }
            } else if vector.velocity < 0.0 {
                if vector.velocity > -time.delta_seconds() * PLAYER_DECEL {
                    vector.velocity = 0.0;
                } else {
                    vector.velocity += time.delta_seconds() * PLAYER_DECEL;
                }
            }
        }
        if vector.velocity > PLAYER_MAX_VELOCITY {
            vector.velocity = PLAYER_MAX_VELOCITY;
        } else if vector.velocity < -PLAYER_MAX_VELOCITY / 2.0 {
            vector.velocity = -PLAYER_MAX_VELOCITY / 2.0;
        }
        if keyboard_input.pressed(KeyCode::Left) {
            vector.direction -= time.delta_seconds() * 2.0;
        } else if keyboard_input.pressed(KeyCode::Right) {
            vector.direction += time.delta_seconds() * 2.0;
        }
        vector.direction = vector.direction % RADIAN_MAX;
    }
}

fn vector_motion(time: Res<Time>, mut transformation_vectors: Query<(&mut Transform, &Vector)>) {
    for (mut transform, vector) in transformation_vectors.iter_mut() {
        transform.translation.y += vector.velocity * time.delta_seconds() * vector.direction.cos();
        transform.translation.x += vector.velocity * time.delta_seconds() * vector.direction.sin();
        transform.rotation = Quat::from_rotation_z(-vector.direction).normalize();
    }
}

fn wrap_screen_edge(
    mut transform_query: Query<&mut Transform>,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    let window = window_query.get_single().unwrap();
    let left_bound = window.width() * -0.05;
    let right_bound = window.width() * 1.05;
    let top_bound = window.height() * 1.05;
    let bottom_bound = window.height() * -0.05;
    for mut transform in transform_query.iter_mut() {
        if transform.translation.x < left_bound {
            transform.translation.x = right_bound;
        } else if transform.translation.x > right_bound {
            transform.translation.x = left_bound;
        }
        if transform.translation.y > top_bound {
            transform.translation.y = bottom_bound;
        } else if transform.translation.y < bottom_bound {
            transform.translation.y = top_bound;
        }
    }
}
