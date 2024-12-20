pub const TICKS_PER_DAY: usize = 1200;
pub const WITCHING_HOUR: usize = TICKS_PER_DAY / 6;
pub const SUNRISE: usize = TICKS_PER_DAY / 4;
pub const SUNSET: usize = TICKS_PER_DAY * 3 / 4;
pub const TICKS_PER_HOUR: usize = TICKS_PER_DAY / 24;
pub const DAYS_PER_MONTH: usize = 28;
pub const DAYS_PER_YEAR: usize = DAYS_PER_MONTH * 12;
pub const DAYS_PER_SEASON: usize = DAYS_PER_YEAR / 4;

#[derive(Debug)]
pub enum Moonphases {
    New,
    WaxingCrescent,
    FirstQuarter,
    WaxingGibbous,
    Full,
    WaningGibbous,
    LastQuarter,
    WaningCrescent,
}

#[derive(Debug)]
pub enum Season {
    Spring,
    Summer,
    Autumn,
    Winter,
}

pub fn get_moon_phase(day: usize) -> Moonphases {
    let moon_day = day % DAYS_PER_MONTH;
    match moon_day / 4 {
        0 => Moonphases::New,
        1 => Moonphases::WaxingCrescent,
        2 => Moonphases::FirstQuarter,
        3 => Moonphases::WaxingGibbous,
        4 => Moonphases::Full,
        5 => Moonphases::WaningGibbous,
        6 => Moonphases::LastQuarter,
        _ => Moonphases::WaningCrescent,
    }
}

pub fn get_season(day: usize) -> Season {
    match day / DAYS_PER_SEASON {
        0 => Season::Spring,
        1 => Season::Summer,
        2 => Season::Autumn,
        _ => Season::Winter,
    }
}
