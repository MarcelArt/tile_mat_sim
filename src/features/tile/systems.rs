use bevy::prelude::*;
use rand::Rng;

use crate::features::tile::{components::{MaterialId, Temperature, Tile}, resources::MaterialRegistry};

fn _temperature_to_color(temp: f32) -> Color {
    // simple mapping: blue (cold) → red (hot)
    let t = (temp.clamp(0.0, 100.0)) / 100.0;
    Color::linear_rgb(t, 0.0, 1.0 - t)
}

pub fn setup_tiles(mut commands: Commands, registry: Res<MaterialRegistry>) {
    let width = 10;
    let height = 10;
    let tile_size = 32.0;
        
    let mut rng = rand::rng();

    for y in 0..height {
        for x in 0..width {
            let materials = [MaterialId::Dirt, MaterialId::Sand, MaterialId::CopperOre];
            let material = materials[((x + y) % materials.len() as i32) as usize];
            let temperature = rng.random_range(10.0..=30.0);
            let data = registry.get(material);

            commands.spawn((
                Sprite {
                    color: data.color,
                    custom_size: Some(Vec2::splat(tile_size - 2.0)),
                    ..default()
                },
                Transform::from_xyz(
                    x as f32 * tile_size, 
                    y as f32 * tile_size, 
                    0.0,
                ),
                Tile { x, y, material },
                Temperature(temperature), // default 20°C
            ));
        }
    }

    info!("Spawned {} tiles", width * height);
}

pub fn debug_tiles(query: Query<(&Tile, &Temperature)>) {
    for (tile, temp) in &query {
        info!("Tile ({}, {}) - Temp: {}", tile.x, tile.y, temp.0);
    }
}