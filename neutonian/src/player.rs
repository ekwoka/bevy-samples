use bevy::{ prelude::*, window::* };
use crate::motion::MotionCharacteristics;
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_startup_system(spawn_player).add_system(player_controls);
    }
}

#[derive(Debug, Component)]
pub struct Player {}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window: Query<&Window, With<PrimaryWindow>>
) {
    let window = window.get_single().expect("Window cannot be undefined");
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
        MotionCharacteristics {
            vector: Vec3::new(0.0, 0.0, 0.0),
            direction: 0.0,
            mass: 1,
        },
    ));
}

const PLAYER_ACCEL: f32 = 480.0;

fn player_controls(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_motion: Query<&mut MotionCharacteristics, With<Player>>,
    time: Res<Time>
) {
    let mut motion = player_motion.single_mut();
    let mut vector = motion.vector;
    let mut direction: f32 = 0.0;
    let rotation_mod = 10.0 * (motion.mass as f32);
    if keyboard_input.pressed(KeyCode::Up) {
        vector += Vec3::new(
            -PLAYER_ACCEL * time.delta_seconds() * motion.direction.sin() * (motion.mass as f32),
            PLAYER_ACCEL * time.delta_seconds() * motion.direction.cos() * (motion.mass as f32),
            0.0
        );
    }
    if keyboard_input.pressed(KeyCode::Down) {
    }
    if keyboard_input.pressed(KeyCode::Left) {
        direction += rotation_mod * time.delta_seconds();
    }
    if keyboard_input.pressed(KeyCode::Right) {
        direction -= rotation_mod * time.delta_seconds();
    }
    motion.direction += direction;
    motion.direction %= crate::motion::RADIAN_MAX;
    motion.vector = vector;
}
