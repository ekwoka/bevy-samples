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

fn setup_env(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
) {
    let window = window_query.get_single().unwrap();
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: asset_server.load("sprites/kenney_simple-space/PNG/Retina/ship_J.png"),
            ..Default::default()
        },
        Player {},
    ));
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 1000.0),
        ..Default::default()
    });
}

fn check_player(time: Res<Time>, mut timer: ResMut<CheckTimer>, query: Query<&Player>) {
    if timer.0.tick(time.delta()).just_finished() {
        for player in query.iter() {
            println!("{:?} is alive!", player);
        }
    }
}

pub struct Boids;

impl Plugin for Boids {
    fn build(&self, app: &mut App) {
        app.insert_resource(CheckTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_startup_system(setup_env)
            .add_system(check_player);
    }
}
