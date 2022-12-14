use colored::Colorize;
use rand::Rng;

const TICKS_PER_DAY: usize = 1200;
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
    pub time_tick: usize,
    pub day: usize,
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
        let height = (size.1 - 1) as usize;
        let random_day = rand::thread_rng().gen_range(0..DAYS_PER_YEAR);
        let random_time = rand::thread_rng().gen_range(0..TICKS_PER_DAY);
        Map {
            day: random_day,
            time_tick: random_time,
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

        if self.time_tick == TICKS_PER_DAY {
            self.time_tick = 0;
            self.day = (self.day + 1) % DAYS_PER_YEAR;
        } else {
            self.time_tick += 1;
        }
    }

    pub fn render(&self, size: (u16, u16)) {
        let mut lines: Vec<String> = vec![];

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
                let is_player_position = position == self.player.position;
                let land = &self.lands[position.1 & self.width + position.0];
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
            lines.push(line);
        }

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

        lines.push(format!(
            "Time {} {} Day {} Moon {} Season {}",
            self.time_tick, time_of_day, self.day, moon_name, season_name
        ));

        // move terminal cursor to top left
        print!("{}", crossterm::cursor::MoveTo(0, 0));
        let all_lines = lines.join("\n\r");
        print!("{}", all_lines);
    }
}
