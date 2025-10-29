use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};

use crate::features::{debug::plugins::DebugPlugin, player_control::plugins::PlayerControlPlugin, tile::plugins::TilePlugin};

mod core;
mod features;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, FrameTimeDiagnosticsPlugin::default()))
        .add_plugins(TilePlugin)
        .add_plugins(PlayerControlPlugin)
        .add_plugins(DebugPlugin)
        .run();
}
