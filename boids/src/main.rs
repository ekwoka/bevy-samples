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
};
use bevy::sprite::SpriteBundle;
use bevy::time::{ Time, Timer, TimerMode };
use bevy::window::{ Window, PrimaryWindow };

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

fn check_player(
    time: Res<Time>,
    mut timer: ResMut<CheckTimer>,
    query: Query<(&Player, &Vector, &Transform)>
) {
    if timer.0.tick(time.delta()).just_finished() {
        if let Ok((player, vector, transform)) = query.get_single() {
            println!("{:?} is alive!", player);
            println!("{:?}", vector);
            println!("{:?}", transform);
        }
    }
}

pub struct Boids;

impl Plugin for Boids {
    fn build(&self, app: &mut App) {
        app.insert_resource(CheckTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_startup_system(setup_env)
            .add_system(check_player)
            .add_system(player_control)
            .add_system(vector_motion)
            .add_system(wrap_screen_edge);
    }
}

const PLAYER_ACCEL: f32 = 160.0;
const PLAYER_DECEL: f32 = 80.0;
const PLAYER_MAX_VELOCITY: f32 = 800.0;

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
        vector.direction = vector.direction % 360.0;
    }
}

fn vector_motion(time: Res<Time>, mut transformation_vectors: Query<(&mut Transform, &Vector)>) {
    for (mut transform, vector) in transformation_vectors.iter_mut() {
        transform.translation.y += vector.velocity * time.delta_seconds() * vector.direction.cos();
        transform.translation.x += vector.velocity * time.delta_seconds() * vector.direction.sin();
        transform.rotation = Quat::from_rotation_z(-vector.direction);
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
            println!("{} < {}", transform.translation.x, left_bound);
            transform.translation.x = right_bound;
        } else if transform.translation.x > right_bound {
            println!("{} > {}", transform.translation.x, right_bound);
            transform.translation.x = left_bound;
        }
        if transform.translation.y > top_bound {
            println!("{} > {}", transform.translation.y, top_bound);
            transform.translation.y = bottom_bound;
        } else if transform.translation.y < bottom_bound {
            println!("{} < {}", transform.translation.y, bottom_bound);
            transform.translation.y = top_bound;
        }
    }
}
