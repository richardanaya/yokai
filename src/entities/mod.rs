mod player;
mod monster;
mod terrain;
mod time;

pub use player::Player;
pub use monster::Monster;
pub use terrain::{Land, LandInstance, LAND_TREE, LAND_DIRT};
pub use time::*;

use colored::Colorize;
const WITCHING_HOUR: usize = TICKS_PER_DAY / 6;
const SUNRISE: usize = TICKS_PER_DAY / 4;
const SUNSET: usize = TICKS_PER_DAY * 3 / 4;
const TICKS_PER_HOUR: usize = TICKS_PER_DAY / 24;
const DAYS_PER_MONTH: usize = 28;
const DAYS_PER_YEAR: usize = DAYS_PER_MONTH * 12;
const DAYS_PER_SEASON: usize = DAYS_PER_YEAR / 4;

enum Moonphases {
    New,
    WaxingCrescent,
    FirstQuarter,
    WaxingGibbous,
    Full,
    WaningGibbous,
    LastQuarter,
    WaningCrescent,
}

enum Season {
    Spring,
    Summer,
    Autumn,
    Winter,
}

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
    pub position: (usize, usize),
    pub hp: u32,
    pub max_hp: u32,
    pub strength: u32,
    pub is_alive: bool,
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

pub struct Map {
    pub time_tick: usize,
    pub day: usize,
    pub width: usize,
    pub height: usize,
    pub player: Player,
    pub items: Vec<Item>,
    pub monsters: Vec<Monster>,
    pub lands: Vec<LandInstance>,
    pub combat_message: Option<String>,
    pub show_stats: bool,
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
    fn pad_stat(value: impl std::fmt::Display, width: usize) -> String {
        let value_str = value.to_string();
        let padding_needed = width.saturating_sub(value_str.len());
        " ".repeat(padding_needed) + &value_str
    }

    pub fn new(size: (u16, u16)) -> Map {
        let width = (size.0 / 2) as usize;
        let height = (size.1 - 1) as usize;
        let random_day = rand::thread_rng().gen_range(0..DAYS_PER_YEAR);
        let random_time = rand::thread_rng().gen_range(0..TICKS_PER_DAY);
        Map {
            day: random_day,
            time_tick: random_time,
            width,
            height,
            combat_message: None,
            show_stats: false,
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
            },
            items: vec![],
            monsters: {
                let mut monsters = Vec::new();
                
                // Add a goblin
                monsters.push(Monster {
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
                });

                // Add an oni
                monsters.push(Monster {
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
                });

                // Add a kappa
                monsters.push(Monster {
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
                });

                monsters
            },
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
        // Check for blocking terrain
        if self.lands[new_position.1 * self.width + new_position.0]
            .template
            .blocking
        {
            return;
        }

        // Check for monster collision
        for monster in &mut self.monsters {
            if monster.position == new_position && monster.is_alive {
                // Combat!
                let player_damage = self.player.strength / 2 + rand::thread_rng().gen_range(1..4);
                monster.hp = monster.hp.saturating_sub(player_damage);

                if monster.hp == 0 {
                    monster.is_alive = false;
                    self.player.exp += 10;
                    self.combat_message = Some(format!(
                        "You hit {} for {} damage and defeated it! (+10 exp)",
                        monster.entity.name.english_name, player_damage
                    ));

                    // Level up at 100 exp
                    if self.player.exp >= 100 {
                        self.player.level += 1;
                        self.player.exp = 0;
                        self.player.max_hp += 5;
                        self.player.hp = self.player.max_hp;
                        self.player.strength += 2;
                        self.combat_message =
                            Some(format!("Level Up! You are now level {}", self.player.level));
                    }
                } else {
                    // Monster counterattack
                    let monster_damage = monster.strength + rand::thread_rng().gen_range(0..2);
                    self.player.hp = self.player.hp.saturating_sub(monster_damage);
                    self.combat_message = Some(format!(
                        "You hit {} for {} damage. It hits back for {} damage!",
                        monster.entity.name.english_name, player_damage, monster_damage
                    ));
                }
                return;
            }
        }

        self.player.position = new_position;

        if self.time_tick == TICKS_PER_DAY {
            self.time_tick = 0;
            self.day = (self.day + 1) % DAYS_PER_YEAR;
        } else {
            self.time_tick += 1;
        }
    }

    pub fn render(&self, size: (u16, u16)) {
        let mut map_lines: Vec<String> = vec![];
        let mut stat_lines: Vec<String> = vec![];

        let moon_day = self.day % DAYS_PER_MONTH;

        let current_moon: Moonphases = match moon_day / 4 {
            0 => Moonphases::New,
            1 => Moonphases::WaxingCrescent,
            2 => Moonphases::FirstQuarter,
            3 => Moonphases::WaxingGibbous,
            4 => Moonphases::Full,
            5 => Moonphases::WaningGibbous,
            6 => Moonphases::LastQuarter,
            _ => Moonphases::WaningCrescent,
        };

        let current_season = match self.day / DAYS_PER_SEASON {
            0 => Season::Spring,
            1 => Season::Summer,
            2 => Season::Autumn,
            _ => Season::Winter,
        };

        let moon_ambient_light_modifier = match current_moon {
            Moonphases::New => 0.0,
            Moonphases::WaxingCrescent => 0.025,
            Moonphases::FirstQuarter => 0.05,
            Moonphases::WaxingGibbous => 0.075,
            Moonphases::Full => 0.1,
            Moonphases::WaningGibbous => 0.075,
            Moonphases::LastQuarter => 0.05,
            Moonphases::WaningCrescent => 0.025,
        };

        let season_ambient_light_modifier = match current_season {
            Season::Spring => (0.975, 1.0, 0.0),
            Season::Summer => (1.0, 0.975, 0.975),
            Season::Autumn => (1.0, 0.985, 0.957),
            Season::Winter => (0.975, 0.975, 1.0),
        };

        let mut time_of_day;
        let mut ambient_color: (f32, f32, f32) = if self.time_tick < WITCHING_HOUR {
            time_of_day = "night";
            (0.3, 0.3, 0.3)
        } else if self.time_tick < WITCHING_HOUR + TICKS_PER_HOUR {
            time_of_day = "witching hour";
            (0.2, 0.2, 0.2)
        } else if self.time_tick < SUNRISE {
            time_of_day = "before sunrise";
            (0.3, 0.3, 0.3)
        } else if self.time_tick < SUNRISE + TICKS_PER_HOUR {
            time_of_day = "sunrise";
            (0.6, 0.6, 0.6)
        } else if self.time_tick < SUNSET - TICKS_PER_HOUR {
            time_of_day = "day";
            (1.0, 1.0, 1.0)
        } else if self.time_tick < SUNSET {
            time_of_day = "sunset";
            (0.6, 0.6, 0.6)
        } else if self.time_tick < SUNSET + TICKS_PER_HOUR / 2 {
            time_of_day = "twilight";
            (0.5, 0.5, 0.5)
        } else {
            time_of_day = "night";
            (0.3, 0.3, 0.3)
        };

        ambient_color = (
            ambient_color.0 * season_ambient_light_modifier.0,
            ambient_color.1 * season_ambient_light_modifier.1,
            ambient_color.2 * season_ambient_light_modifier.2,
        );

        if self.time_tick < SUNRISE + TICKS_PER_HOUR || self.time_tick > SUNSET - TICKS_PER_HOUR {
            ambient_color.0 += moon_ambient_light_modifier;
            ambient_color.1 += moon_ambient_light_modifier;
            ambient_color.2 += moon_ambient_light_modifier;
        }

        for y in 0..self.height {
            let mut line = String::new();
            for x in 0..self.width {
                let position = (x, y);
                // Check for living monsters first
                let mut is_monster = false;
                for monster in &self.monsters {
                    if monster.position == position && monster.is_alive {
                        let true_color = monster.entity.colors[0];
                        let r = ((true_color >> 16) as u8 as f32 * ambient_color.0) as u8;
                        let g = ((true_color >> 8) as u8 as f32 * ambient_color.1) as u8;
                        let b = ((true_color) as u8 as f32 * ambient_color.2) as u8;
                        line.push_str(&format!(
                            "{}",
                            &(monster.entity.symbols[0].to_string().truecolor(r, g, b))
                        ));
                        is_monster = true;
                        break;
                    }
                }

                if is_monster {
                    continue;
                }

                let is_player_position = position == self.player.position;
                let land = &self.lands[position.1 * self.width + position.0];
                if is_player_position {
                    let true_color = self.player.entity.colors[1];
                    let r = ((true_color >> 16) as u8 as f32 * ambient_color.0) as u8;
                    let g = ((true_color >> 8) as u8 as f32 * ambient_color.1) as u8;
                    let b = ((true_color) as u8 as f32 * ambient_color.2) as u8;
                    line.push_str(&format!(
                        "{}",
                        &(self.player.entity.symbols[0].to_string().truecolor(r, g, b))
                    ));
                } else {
                    let true_color = land.template.entity.colors[land.color_index];
                    let r = ((true_color >> 16) as u8 as f32 * ambient_color.0) as u8;
                    let g = ((true_color >> 8) as u8 as f32 * ambient_color.1) as u8;
                    let b = ((true_color) as u8 as f32 * ambient_color.2) as u8;
                    line.push_str(&format!(
                        "{}",
                        &(land.template.entity.symbols[land.symbol_index]
                            .to_string()
                            .truecolor(r, g, b))
                    ));
                }
            }
            map_lines.push(line);
        }

        // Add character stats
        stat_lines.push("╔═══════════════════╗".to_string());
        stat_lines.push("║  Character Sheet  ║".to_string());
        stat_lines.push("╠═══════════════════╣".to_string());
        stat_lines.push(format!(
            "║ Level: {}        ║",
            Self::pad_stat(self.player.level, 3)
        ));
        stat_lines.push(format!(
            "║ EXP: {}/100      ║",
            Self::pad_stat(self.player.exp, 3)
        ));
        stat_lines.push("╟───────────────────╢".to_string());
        stat_lines.push(format!(
            "║ HP: {}/{}       ║",
            Self::pad_stat(self.player.hp, 3),
            Self::pad_stat(self.player.max_hp, 3)
        ));
        stat_lines.push(format!(
            "║ MP: {}/{}       ║",
            Self::pad_stat(self.player.mp, 3),
            Self::pad_stat(self.player.max_mp, 3)
        ));
        stat_lines.push("╟───────────────────╢".to_string());
        stat_lines.push("║      Stats        ║".to_string());
        stat_lines.push("╟───────────────────╢".to_string());
        stat_lines.push(format!(
            "║ STR: {}          ║",
            Self::pad_stat(self.player.strength, 3)
        ));
        stat_lines.push(format!(
            "║ DEX: {}          ║",
            Self::pad_stat(self.player.dexterity, 3)
        ));
        stat_lines.push(format!(
            "║ CON: {}          ║",
            Self::pad_stat(self.player.constitution, 3)
        ));
        stat_lines.push(format!(
            "║ INT: {}          ║",
            Self::pad_stat(self.player.intelligence, 3)
        ));
        stat_lines.push(format!(
            "║ WIS: {}          ║",
            Self::pad_stat(self.player.wisdom, 3)
        ));
        stat_lines.push(format!(
            "║ CHA: {}          ║",
            Self::pad_stat(self.player.charisma, 3)
        ));

        let moon_name = match current_moon {
            Moonphases::New => "New Moon",
            Moonphases::WaxingCrescent => "Waxing Crescent",
            Moonphases::FirstQuarter => "First Quarter",
            Moonphases::WaxingGibbous => "Waxing Gibbous",
            Moonphases::Full => "Full Moon",
            Moonphases::WaningGibbous => "Waning Gibbous",
            Moonphases::LastQuarter => "Last Quarter",
            Moonphases::WaningCrescent => "Waning Crescent",
        };

        let season_name = match current_season {
            Season::Spring => "Spring",
            Season::Summer => "Summer",
            Season::Autumn => "Autumn",
            Season::Winter => "Winter",
        };

        // Add time and environmental information to stat lines
        stat_lines.push("╟───────────────────╢".to_string());
        stat_lines.push("║    Environment    ║".to_string());
        stat_lines.push("╟───────────────────╢".to_string());
        stat_lines.push(format!("║ Time: {:10}  ║", self.time_tick));
        stat_lines.push(format!("║ Period: {:8}  ║", time_of_day));
        stat_lines.push(format!("║ Day: {:11}  ║", self.day));
        stat_lines.push(format!("║ Moon: {:10}  ║", moon_name));
        stat_lines.push(format!("║ Season: {:8}  ║", season_name));
        stat_lines.push("╚═══════════════════╝".to_string());

        // move terminal cursor to top left
        print!("{}", crossterm::cursor::MoveTo(0, 0));
        // First print the map
        print!("{}", crossterm::cursor::MoveTo(0, 0));
        for line in map_lines {
            print!("{}\n\r", line);
        }

        // Then overlay the combat message if any
        if let Some(msg) = &self.combat_message {
            print!(
                "{}{}",
                crossterm::cursor::MoveTo(0, (self.height + 1) as u16),
                format!("Combat: {}", msg)
            );
        }

        // Then overlay the stats if enabled
        if self.show_stats {
            let stats_x = 0; // Offset from left edge
            for (i, stat_line) in stat_lines.iter().enumerate() {
                print!(
                    "{}{}",
                    crossterm::cursor::MoveTo(stats_x, i as u16),
                    stat_line
                );
            }
        }
    }
}
