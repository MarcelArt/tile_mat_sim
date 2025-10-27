use bevy::prelude::*;

use crate::features::{player_control::plugins::PlayerControlPlugin, tile::plugins::TilePlugin};

mod core;
mod features;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TilePlugin)
        .add_plugins(PlayerControlPlugin)
        .run();
}
