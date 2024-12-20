use bevy::prelude::*;
use super::{Entity, EntityName};
use rand::Rng;

pub struct Land {
    pub entity: Entity,
    pub blocking: bool,
}

pub struct LandInstance {
    pub template: &'static Land,
    pub symbol_index: usize,
    pub color_index: usize,
    pub position: (usize, usize),
}

pub const LAND_TREE: Land = Land {
    entity: Entity {
        name: EntityName {
            english_name: "Land",
            japanese_name: Some("地"),
            discovered: true,
            use_japanese: true,
        },
        description: "You are on land",
        symbols: ["木", "林", "森"],
        colors: [0x34623f, 0x607744, 0x31572c],
        opacity: 255,
        visibly_blocking: false,
    },
    blocking: false,
};

pub const LAND_DIRT: Land = Land {
    entity: Entity {
        name: EntityName {
            english_name: "Land",
            japanese_name: Some("地"),
            discovered: true,
            use_japanese: true,
        },
        description: "You are on land",
        symbols: [". ", " .", ".."],
        colors: [0x806443, 0x685634, 0x806443],
        opacity: 255,
        visibly_blocking: false,
    },
    blocking: false,
};

impl LandInstance {
    pub fn generate_terrain(width: usize, height: usize) -> Vec<LandInstance> {
        let size = width * height;
        let mut terrain = Vec::with_capacity(size);
        
        for i in 0..size {
            if rand::thread_rng().gen::<f32>() < 0.3 {
                terrain.push(LandInstance {
                    template: &LAND_TREE,
                    symbol_index: rand::thread_rng().gen_range(0..LAND_TREE.entity.symbols.len()),
                    color_index: rand::thread_rng().gen_range(0..LAND_TREE.entity.colors.len()),
                    position: (i % width, i / width),
                });
            } else {
                terrain.push(LandInstance {
                    template: &LAND_DIRT,
                    symbol_index: rand::thread_rng().gen_range(0..LAND_DIRT.entity.symbols.len()),
                    color_index: rand::thread_rng().gen_range(0..LAND_DIRT.entity.colors.len()),
                    position: (i % width, i / width),
                });
            }
        }
        terrain
    }
}
