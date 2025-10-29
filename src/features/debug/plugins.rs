use bevy::prelude::*;

use crate::features::debug::systems::{spawn_tile_info_hud, update_tile_info_hud};

pub struct DebugPlugin;
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_tile_info_hud);
        app.add_systems(Update, update_tile_info_hud);
    }
}