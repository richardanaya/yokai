use bevy::prelude::*;
use bevy::input::keyboard::KeyCode;

#[derive(Component, Clone)]
pub struct MapItem {
    pub character: String,
    pub color: Color,
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerBody {
    pub character: String,
}

#[derive(Component)]
pub struct PlayerWeapon {
    pub character: String,
}

#[derive(Component)]
pub struct MapPosition {
    pub x: i32,
    pub y: i32,
    pub z: i32,  // Stack position
}

#[derive(Resource)]
pub struct GameMap {
    pub width: i32,
    pub height: i32,
    pub items: Vec<Vec<Vec<Entity>>>,  // [x][y][stack_position]
}

impl GameMap {
    pub fn new(width: i32, height: i32) -> Self {
        let items = vec![vec![Vec::new(); height as usize]; width as usize];
        Self {
            width,
            height,
            items,
        }
    }

    pub fn add_item(&mut self, x: i32, y: i32, entity: Entity) {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            self.items[x as usize][y as usize].push(entity);
        }
    }
}
