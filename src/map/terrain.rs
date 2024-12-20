use crate::components::MapItem;
use bevy::prelude::*;

#[derive(Clone)]
pub struct TerrainType {
    pub characters: Vec<&'static str>,
    pub colors: Vec<Color>,
    pub solid: bool,
}

impl TerrainType {
    pub fn is_solid(&self) -> bool {
        self.solid
    }

    pub fn to_map_item(&self) -> MapItem {
        MapItem::new(
            self.characters.iter().map(|&s| s.to_string()).collect(),
            self.colors.clone(),
            self.solid,
        )
    }
}

pub fn grass() -> TerrainType {
    TerrainType {
        characters: vec!["'", ",", "."],
        colors: vec![
            Color::srgb(0.2, 0.6, 0.2),
            Color::srgb(0.3, 0.5, 0.2),
            Color::srgb(0.25, 0.55, 0.25),
        ],
        solid: false,
    }
}

pub fn rock() -> TerrainType {
    TerrainType {
        characters: vec!["石", "岩", "磐"],
        colors: vec![
            Color::srgb(0.5, 0.5, 0.5),
            Color::srgb(0.4, 0.4, 0.4),
            Color::srgb(0.45, 0.45, 0.45),
        ],
        solid: true,
    }
}

pub fn tree() -> TerrainType {
    TerrainType {
        characters: vec!["木", "林", "森"],
        colors: vec![
            Color::srgb(0.1, 0.4, 0.1),
            Color::srgb(0.15, 0.45, 0.15),
            Color::srgb(0.2, 0.5, 0.2),
        ],
        solid: true,
    }
}

pub fn earth() -> TerrainType {
    TerrainType {
        characters: vec![".", ",", "'"],
        colors: vec![
            Color::srgb(0.6, 0.4, 0.2),
            Color::srgb(0.55, 0.35, 0.15),
            Color::srgb(0.5, 0.3, 0.1),
        ],
        solid: false,
    }
}
