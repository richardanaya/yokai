use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct MapItem {
    pub character_variants: Vec<String>,
    pub color_variants: Vec<Color>,
    pub current_variant: usize,
}

impl MapItem {
    pub fn new(characters: Vec<String>, colors: Vec<Color>) -> Self {
        let variant_count = characters.len().min(colors.len());
        assert!(variant_count > 0, "Must provide at least one variant");
        Self {
            character_variants: characters,
            color_variants: colors,
            current_variant: rand::random::<usize>() % variant_count,
        }
    }

    pub fn current_character(&self) -> &str {
        &self.character_variants[self.current_variant]
    }

    pub fn current_color(&self) -> Color {
        self.color_variants[self.current_variant]
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerStats {
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
    pub show_inventory: bool,
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self {
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
            show_inventory: false,
        }
    }
}

#[derive(Component)]
#[derive(Component)]
pub struct PlayerBody;

#[derive(Component)]
#[derive(Component)]
pub struct PlayerWeapon;

#[derive(Component)]
#[derive(Component)]
#[allow(dead_code)]
pub struct MapPosition {
    pub x: i32,
    pub y: i32,
    pub z: i32,  // Stack position
}

#[derive(Resource)]
#[derive(Resource)]
#[allow(dead_code)]
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
