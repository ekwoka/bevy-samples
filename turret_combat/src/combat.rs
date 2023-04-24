use crate::motion;
use bevy::prelude::*;
use bevy::window::{PrimaryWindow, Window};
use rand::prelude::random;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
            .add_system(spawn_asteroids);
    }
}

#[derive(Resource)]
struct SpawnTimer(Timer);

fn spawn_asteroids(
    mut commands: Commands,
    mut spawn_timer: ResMut<SpawnTimer>,
    window: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
) {
    if spawn_timer.0.tick(time.delta()).just_finished() {
        let window = window.get_single().expect("Window cannot be undefined");
        let motion_vector = random_vector() * (random::<f32>() * (80.0 + 40.0));
        let position_vector = screen_edge_at_angle(
            random::<f32>() * motion::RADIAN_MAX,
            window.width(),
            window.height(),
        );

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_translation(position_vector)
                    .with_scale([0.25, 0.25, 1.0].into()),
                texture: asset_server
                    .load("sprites/kenney_simple-space/PNG/Retina/meteor_detailedSmall.png"),
                ..Default::default()
            },
            motion::MotionCharacteristics {
                vector: motion_vector,
                direction: random::<f32>() * motion::RADIAN_MAX,
                mass: 0.5,
            },
        ));
    }
}

fn random_vector() -> Vec3 {
    let angle = random::<f32>() * motion::RADIAN_MAX;
    Vec3::new(angle.sin(), angle.cos(), 0.0)
}

fn screen_edge_at_angle(angle: f32, window_width: f32, window_height: f32) -> Vec3 {
    let window_center = Vec3::new(window_width / 2.0, window_height / 2.0, 0.0);
    let tangent = angle.tan();
    let x = window_width / 2.0;
    let y = window_height / 2.0;
    let x_intercept = x * tangent;
    let y_intercept = y / tangent;
    Vec3::new(y_intercept, x_intercept, 0.0).clamp(Vec3::new(-x, -y, 0.0), Vec3::new(x, y, 0.0))
        + window_center
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_screen_edge_at_90_angle() {
        let position = screen_edge_at_angle(1.0 / 4.0 * motion::RADIAN_MAX, 800.0, 600.0);
        assert_eq!(position, Vec3::new(400.0, 0.0, 0.0));
    }

    #[test]
    fn test_screen_edge_at_45_angle() {
        let position = screen_edge_at_angle(1.0 / 8.0 * motion::RADIAN_MAX, 800.0, 600.0);
        assert_eq!(position, Vec3::new(700.0, 600.0, 0.0));
    }

    #[test]
    fn test_screen_edge_at_30_angle() {
        let position = screen_edge_at_angle(1.0 / 12.0 * motion::RADIAN_MAX, 800.0, 600.0);
        assert_eq!(position, Vec3::new(800.0, 530.9401, 0.0));
    }
}
