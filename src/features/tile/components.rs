use bevy::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MaterialId {
    Dirt,
    Sand,
    Sandstone,
    CopperOre,
}

#[derive(Clone, Debug)]
pub struct MaterialData {
    pub name: &'static str,
    pub color: Color,
    pub conductivity: f32, // how well it conducts heat
    pub insulation: f32,   // how well it resists heat
    pub hardness: f32,
}

#[derive(Component)]
pub struct Tile {
    pub x: i32,
    pub y: i32,
    pub material: MaterialId,
}

#[derive(Component)]
pub struct Temperature(pub f32);