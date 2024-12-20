use bevy::prelude::*;
use rand::Rng;

pub struct TerrainVariant {
    pub character: &'static str,
    pub color: Color,
}

pub struct TerrainType {
    pub name: &'static str,
    pub variants: &'static [TerrainVariant],
}

impl TerrainType {
    pub fn random_variant(&self) -> &TerrainVariant {
        let mut rng = rand::thread_rng();
        &self.variants[rng.gen_range(0..self.variants.len())]
    }
}

pub const GRASS: TerrainType = TerrainType {
    name: "Grass",
    variants: &[
        TerrainVariant {
            character: "草",
            color: Color::rgb(0.2, 0.6, 0.2),
        },
        TerrainVariant {
            character: "艸",
            color: Color::rgb(0.3, 0.5, 0.2),
        },
        TerrainVariant {
            character: "茸",
            color: Color::rgb(0.25, 0.55, 0.25),
        },
    ],
};

pub const ROCK: TerrainType = TerrainType {
    name: "Rock",
    variants: &[
        TerrainVariant {
            character: "石",
            color: Color::rgb(0.5, 0.5, 0.5),
        },
        TerrainVariant {
            character: "岩",
            color: Color::rgb(0.4, 0.4, 0.4),
        },
        TerrainVariant {
            character: "磐",
            color: Color::rgb(0.45, 0.45, 0.45),
        },
    ],
};

pub const TREE: TerrainType = TerrainType {
    name: "Tree",
    variants: &[
        TerrainVariant {
            character: "木",
            color: Color::rgb(0.1, 0.4, 0.1),
        },
        TerrainVariant {
            character: "林",
            color: Color::rgb(0.15, 0.45, 0.15),
        },
        TerrainVariant {
            character: "森",
            color: Color::rgb(0.2, 0.5, 0.2),
        },
    ],
};
