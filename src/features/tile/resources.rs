use std::collections::HashMap;

use bevy::prelude::*;

use crate::features::tile::components::{MaterialData, MaterialId};

#[derive(Resource)]
pub struct MaterialRegistry {
    pub materials: HashMap<MaterialId, MaterialData>,
}

impl MaterialRegistry {
    pub fn new() -> Self {
        let mut materials = HashMap::new();
        materials.insert(
            MaterialId::Dirt,
            MaterialData {
                name: "Dirt",
                color: Color::linear_rgb(0.5, 0.35, 0.2),
                conductivity: 0.4,
                insulation: 0.8,
                hardness: 3.0,
            },
        );
        materials.insert(
            MaterialId::Sand,
            MaterialData {
                name: "Sand",
                color: Color::linear_rgb(0.9, 0.85, 0.6),
                conductivity: 0.6,
                insulation: 0.5,
                hardness: 2.0,
            },
        );
        materials.insert(
            MaterialId::CopperOre,
            MaterialData {
                name: "Copper Ore",
                color: Color::linear_rgb(0.7, 0.3, 0.1),
                conductivity: 1.5,
                insulation: 0.2,
                hardness: 8.0,
            },
        );
        Self { materials }
    }

    pub fn get(&self, id: MaterialId) -> &MaterialData {
        self.materials.get(&id).unwrap()
    }
}
