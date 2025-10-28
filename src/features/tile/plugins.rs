use bevy::prelude::*;

use crate::features::tile::{resources::{MaterialRegistry, TileGridConfig}, systems::{diffuse_heat, setup_tiles}};

pub struct TilePlugin;
impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MaterialRegistry::new());
        app.insert_resource(TileGridConfig::default());
        app.add_systems(Startup, setup_tiles);
        app.add_systems(Update, diffuse_heat);
    }
}