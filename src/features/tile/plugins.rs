use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::features::tile::{resources::MaterialRegistry, systems::{debug_tiles, setup_tiles}};

pub struct TilePlugin;
impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MaterialRegistry::new());
        app.add_systems(Startup, setup_tiles);
        app.add_systems(Update, debug_tiles.run_if(input_just_pressed(KeyCode::Space)));
    }
}