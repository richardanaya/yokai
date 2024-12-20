use bevy::prelude::*;
use super::{Entity, EntityName};

pub struct Monster {
    pub entity: Entity,
    pub position: (usize, usize),
    pub hp: u32,
    pub max_hp: u32,
    pub strength: u32,
    pub is_alive: bool,
}

impl Monster {
    pub fn new_goblin(width: usize, height: usize) -> Self {
        Monster {
            entity: Entity {
                name: EntityName {
                    english_name: "Goblin",
                    japanese_name: Some("ゴブリン"),
                    discovered: true,
                    use_japanese: true,
                },
                description: "A nasty goblin",
                symbols: ["G ", "g ", "G!"],
                colors: [0x00FF00, 0x32CD32, 0x228B22],
                opacity: 255,
                visibly_blocking: true,
            },
            position: (width / 2 + 3, height / 2),
            hp: 10,
            max_hp: 10,
            strength: 3,
            is_alive: true,
        }
    }

    pub fn new_oni(width: usize, height: usize) -> Self {
        Monster {
            entity: Entity {
                name: EntityName {
                    english_name: "Oni",
                    japanese_name: Some("鬼"),
                    discovered: true,
                    use_japanese: true,
                },
                description: "A fearsome demon",
                symbols: ["鬼", "鬼", "鬼"],
                colors: [0xFF0000, 0xCC0000, 0x990000],
                opacity: 255,
                visibly_blocking: true,
            },
            position: (width / 2 - 3, height / 2),
            hp: 20,
            max_hp: 20,
            strength: 5,
            is_alive: true,
        }
    }

    pub fn new_kappa(width: usize, height: usize) -> Self {
        Monster {
            entity: Entity {
                name: EntityName {
                    english_name: "Kappa",
                    japanese_name: Some("河童"),
                    discovered: true,
                    use_japanese: true,
                },
                description: "A water imp",
                symbols: ["河", "童", "河"],
                colors: [0x0000FF, 0x0000CC, 0x000099],
                opacity: 255,
                visibly_blocking: true,
            },
            position: (width / 2, height / 2 + 3),
            hp: 15,
            max_hp: 15,
            strength: 4,
            is_alive: true,
        }
    }
}
