use bevy::prelude::*;
use super::{Entity, EntityName};

pub struct Player {
    pub entity: Entity,
    pub position: (usize, usize),
    pub level: u32,
    pub exp: u32,
    pub hp: u32,
    pub max_hp: u32,
    pub mp: u32,
    pub max_mp: u32,
    pub strength: u32,
    pub dexterity: u32,
    pub constitution: u32,
    pub intelligence: u32,
    pub wisdom: u32,
    pub charisma: u32,
}

impl Player {
    pub fn new(width: usize, height: usize) -> Self {
        Player {
            entity: Entity {
                name: EntityName {
                    english_name: "Player",
                    japanese_name: Some("プレイヤー"),
                    discovered: true,
                    use_japanese: true,
                },
                description: "You are the player",
                symbols: ["@/", "\\@", "@|"],
                colors: [0xffedd8, 0xf3d5b5, 0xe7bc91],
                opacity: 255,
                visibly_blocking: true,
            },
            position: (width / 2, height / 2), // Start in center
            level: 1,
            exp: 0,
            hp: 20,
            max_hp: 20,
            mp: 10,
            max_mp: 10,
            strength: 10,
            dexterity: 10,
            constitution: 10,
            intelligence: 10,
            wisdom: 10,
            charisma: 10,
        }
    }
}
