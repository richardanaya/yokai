use colored::Colorize;
use rand::Rng;

pub struct EntityName {
    pub english_name: String,
    pub japanese_name: Option<String>,
    pub discovered: bool,
    pub use_japanese: bool,
}

pub struct Entity {
    pub name: EntityName,
    pub description: String,
    pub symbol: char,
    pub colors: Vec<u32>,
    pub opacity: u8,
    pub visibly_blocking: bool,
    pub position: (usize, usize),
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

pub struct Player {
    pub entity: Entity,
}

pub struct Map {
    pub width: usize,
    pub height: usize,
    pub player: Player,
    pub items: Vec<Item>,
    pub monsters: Vec<Monster>,
    pub lands: Vec<Land>,
}

impl Map {
    pub fn new(size: (u16, u16)) -> Map {
        let width = size.0 as usize;
        let height = size.1 as usize;
        Map {
            width,
            height,
            player: Player {
                entity: Entity {
                    name: EntityName {
                        english_name: "Player".to_string(),
                        japanese_name: Some("プレイヤー".to_string()),
                        discovered: true,
                        use_japanese: true,
                    },
                    description: "You are the player".to_string(),
                    symbol: '@',
                    colors: vec![0x00FF00],
                    opacity: 255,
                    visibly_blocking: true,
                    position: (0, 0),
                },
            },
            items: vec![],
            monsters: vec![],
            lands: {
                let size = (width * height) as usize;
                let mut zero_vec: Vec<Land> = Vec::with_capacity(size as usize);
                for i in 0..size {
                    zero_vec.push(Land {
                        entity: Entity {
                            name: EntityName {
                                english_name: "Land".to_string(),
                                japanese_name: Some("地".to_string()),
                                discovered: true,
                                use_japanese: true,
                            },
                            description: "You are on land".to_string(),
                            symbol: '.',
                            colors: vec![0xFFFFFF, 0xDFDFDF, 0xCECECE],
                            opacity: 255,
                            visibly_blocking: false,
                            position: (i % width, i / height),
                        },
                        blocking: false,
                    });
                }
                zero_vec
            },
        }
    }

    pub fn move_player(&mut self, dx: i32, dy: i32) {
        let (player_x, player_y) = self.player.entity.position;
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
        if self.lands[new_position.1 & self.width + new_position.0].blocking {
            return;
        }
        self.player.entity.position = new_position;
    }

    pub fn render(&self, size: (u16, u16)) {
        let mut lines: Vec<String> = vec![];
        for y in 0..self.height {
            let mut line = String::new();
            for x in 0..self.width {
                let position = (x, y);
                let is_player_position = position == self.player.entity.position;
                let land = &self.lands[position.1 & self.width + position.0];
                if is_player_position {
                    let true_color = land.entity.colors[0];
                    let r = (true_color >> 16) as u8;
                    let g = ((true_color >> 8) & 0xFF) as u8;
                    let b = (true_color & 0xFF) as u8;
                    line.push_str(&format!(
                        "{}",
                        &(self.player.entity.symbol.to_string().truecolor(r, g, b))
                    ));
                } else {
                    let random_true_color = land.entity.colors
                        [rand::thread_rng().gen_range(0..land.entity.colors.len())];
                    let r = (random_true_color >> 16) as u8;
                    let g = ((random_true_color >> 8) & 0xFF) as u8;
                    let b = (random_true_color & 0xFF) as u8;
                    line.push_str(&format!(
                        "{}",
                        &(land.entity.symbol.to_string().truecolor(r, g, b))
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
