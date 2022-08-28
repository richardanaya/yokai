use colored::Colorize;
use rand::Rng;

pub struct EntityName {
    pub english_name: &'static str,
    pub japanese_name: Option<&'static str>,
    pub discovered: bool,
    pub use_japanese: bool,
}

pub struct Entity {
    pub name: EntityName,
    pub description: &'static str,
    pub symbols: [&'static str; 3],
    pub colors: [u32; 3],
    pub opacity: u8,
    pub visibly_blocking: bool,
}

pub struct Monster {
    pub entity: Entity,
}

pub struct Item {
    pub entity: Entity,
}

pub struct Land {
    pub entity: Entity,
    pub blocking: bool,
}

pub struct LandInstance {
    template: &'static Land,
    symbol_index: usize,
    color_index: usize,
    pub position: (usize, usize),
}

pub struct Player {
    pub entity: Entity,
    pub position: (usize, usize),
}

pub struct Map {
    pub width: usize,
    pub height: usize,
    pub player: Player,
    pub items: Vec<Item>,
    pub monsters: Vec<Monster>,
    pub lands: Vec<LandInstance>,
}

const LAND_TREE: Land = Land {
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

const LAND_DIRT: Land = Land {
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

impl Map {
    pub fn new(size: (u16, u16)) -> Map {
        let width = (size.0 / 2) as usize;
        let height = size.1 as usize;
        Map {
            width,
            height,
            player: Player {
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
                position: (0, 0),
            },
            items: vec![],
            monsters: vec![],
            lands: {
                let size = (width * height) as usize;
                let mut zero_vec: Vec<LandInstance> = Vec::with_capacity(size as usize);
                for i in 0..size {
                    if rand::thread_rng().gen::<f32>() < 0.3 {
                        zero_vec.push(LandInstance {
                            template: &LAND_TREE,
                            symbol_index: rand::thread_rng()
                                .gen_range(0..LAND_TREE.entity.symbols.len()),
                            color_index: rand::thread_rng()
                                .gen_range(0..LAND_TREE.entity.colors.len()),
                            position: (i % width, i / width),
                        });
                    } else {
                        zero_vec.push(LandInstance {
                            template: &LAND_DIRT,
                            symbol_index: rand::thread_rng()
                                .gen_range(0..LAND_TREE.entity.symbols.len()),
                            color_index: rand::thread_rng()
                                .gen_range(0..LAND_TREE.entity.colors.len()),
                            position: (i % width, i / width),
                        });
                    }
                }
                zero_vec
            },
        }
    }

    pub fn move_player(&mut self, dx: i32, dy: i32) {
        let (player_x, player_y) = self.player.position;
        let mut new_x = player_x as i32 + dx;
        let mut new_y = player_y as i32 + dy;
        if new_x <= 0 {
            new_x = 0;
        }
        if new_y <= 0 {
            new_y = 0;
        }
        if new_x >= self.width as i32 {
            new_x = self.width as i32 - 1;
        }
        if new_y >= self.height as i32 {
            new_y = self.height as i32 - 1;
        }
        let new_position: (usize, usize) = (new_x as usize, new_y as usize);
        if self.lands[new_position.1 & self.width + new_position.0]
            .template
            .blocking
        {
            return;
        }
        self.player.position = new_position;
    }

    pub fn render(&self, size: (u16, u16)) {
        let mut lines: Vec<String> = vec![];
        for y in 0..self.height {
            let mut line = String::new();
            for x in 0..self.width {
                let position = (x, y);
                let is_player_position = position == self.player.position;
                let land = &self.lands[position.1 & self.width + position.0];
                if is_player_position {
                    let true_color = self.player.entity.colors[1];
                    let r = (true_color >> 16) as u8;
                    let g = ((true_color >> 8) & 0xFF) as u8;
                    let b = (true_color & 0xFF) as u8;
                    line.push_str(&format!(
                        "{}",
                        &(self.player.entity.symbols[0].to_string().truecolor(r, g, b))
                    ));
                } else {
                    let true_color = land.template.entity.colors[land.color_index];
                    let r = (true_color >> 16) as u8;
                    let g = ((true_color >> 8) & 0xFF) as u8;
                    let b = (true_color & 0xFF) as u8;
                    line.push_str(&format!(
                        "{}",
                        &(land.template.entity.symbols[land.symbol_index]
                            .to_string()
                            .truecolor(r, g, b))
                    ));
                }
            }
            lines.push(line);
        }
        // move terminal cursor to top left
        print!("{}", crossterm::cursor::MoveTo(0, 0));
        let all_lines = lines.join("\n\r");
        print!("{}", all_lines);
    }
}
