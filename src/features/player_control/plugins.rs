use bevy::prelude::*;

use crate::features::player_control::systems::{camera_movement, click_tile, setup_camera};

pub struct PlayerControlPlugin;
impl Plugin for PlayerControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
        app.add_systems(Update, (click_tile, camera_movement));
    }
}