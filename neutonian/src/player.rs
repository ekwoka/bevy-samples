use bevy::prelude::*;
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_startup_system(spawn_player).add_system(player_control);
    }
}
