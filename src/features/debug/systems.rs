use bevy::{diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin}, prelude::*, window::PrimaryWindow};

use crate::features::{debug::components::TileInfoText, tile::{components::{Temperature, Tile}, resources::TileGridConfig}};

pub fn spawn_tile_info_hud(mut commands: Commands) {
    commands.spawn((
        Text::new("Test"),
        Node {
            position_type: PositionType::Absolute,
            top: px(5),
            left: px(15),
            ..default()
        },
        TileInfoText,
    ));
}

pub fn update_tile_info_hud(
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &GlobalTransform)>,
    tiles: Query<(&Tile, &Temperature)>,
    config: Res<TileGridConfig>,
    mut text: Single<&mut Text, With<TileInfoText>>,
    diagnostics: Res<DiagnosticsStore>,
) {
    let (camera, cam_transform) = *camera;

    let fps = match diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
        Some(d) => d.smoothed().unwrap_or_default(),
        None => 0.0,
    };

    text.0 = format!(
        "FPS: {:.0}",
        fps,
    );

    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };

    let Ok(world_pos) = camera.viewport_to_world(cam_transform, cursor_pos)
        .map(|ray| ray.origin.truncate()) else {
            return;
        };

    let tile_x = (world_pos.x / config.tile_size).round() as i32;
    let tile_y = (world_pos.y / config.tile_size).round() as i32;

    // let (tiles, temperatures) = *tiles;
    if let Some((tile, temperature)) = tiles.iter().find(|(t, _)| t.x == tile_x && t.y == tile_y) {
        text.0 = format!(
            "FPS: {:.0}\nTile: {:?} ({}, {})\nTemperature: {:.2} C",
            fps, tile.material, tile.x, tile.y, temperature.0
        )
    }
}