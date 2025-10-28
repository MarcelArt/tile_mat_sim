use std::collections::HashMap;

use bevy::prelude::*;
use rand::Rng;

use crate::features::tile::{components::{MaterialId, Temperature, Tile}, resources::{MaterialRegistry, TileGridConfig}};

fn _temperature_to_color(temp: f32) -> Color {
    // simple mapping: blue (cold) → red (hot)
    let t = (temp.clamp(0.0, 100.0)) / 100.0;
    Color::linear_rgb(t, 0.0, 1.0 - t)
}

pub fn setup_tiles(mut commands: Commands, registry: Res<MaterialRegistry>, config: Res<TileGridConfig>) {
    let width = config.width;
    let height = config.height;
    let tile_size = config.tile_size;
        
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

pub fn diffuse_heat(
    tiles_query: Query<(&Tile, &mut Temperature)>,
) {
    let mut new_temps = HashMap::new();

    let current: HashMap<(i32, i32), f32> = tiles_query.iter()
        .map(|(tile, temp)| ((tile.x, tile.y), temp.0))
        .collect();

    for (tile, temperature) in &tiles_query {
        let mut sum  = 0.0;
        let mut count = 0;

        for offset in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let k = (tile.x + offset.0, tile.y + offset.1);
            if let Some(temp) = current.get(&k) {
                sum += *temp;
                count += 1;
            }
        }

        let new_temp = if count > 0 {
           (temperature.0 * 3.0 + (sum / count as f32))  / 4.0
        } else {
            temperature.0
        };

        new_temps.insert((tile.x, tile.y), new_temp);
    }

    for (tile, mut temperature) in tiles_query {
        if let Some(&new_temp) = new_temps.get(&(tile.x, tile.y)) {
            temperature.0 = new_temp;
        }
    }
}

pub fn _debug_tiles(query: Query<(&Tile, &Temperature)>) {
    for (tile, temp) in &query {
        info!("Tile ({}, {}) - Temp: {}", tile.x, tile.y, temp.0);
    }
}