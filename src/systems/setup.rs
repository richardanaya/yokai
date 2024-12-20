use bevy::prelude::*;
use crate::components::*;
use rand::Rng;

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    // Camera
    commands.spawn((Camera2d::default(), MainCamera));

    // Load the font
    let font = asset_server.load("fonts/NotoSansJP-VariableFont_wght.ttf");

    // Get window dimensions
    let window = window_query.single();
    let width = window.width();
    let height = window.height();

    // Character size and spacing
    let char_size = 12.0;
    let spacing = char_size;

    // Calculate starting position (top-left corner)
    let start_x = -width / 2.0 + spacing / 2.0;
    let start_y = height / 2.0 - spacing / 2.0;

    spawn_monsters(&mut commands, &font, start_x, start_y, spacing);
    spawn_combat_message(&mut commands, &font, height);
    spawn_terrain(&mut commands, &font, width, height, spacing);
}

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.single();
    let char_size = 12.0;
    // Calculate starting position (top-left corner)
    let start_x = -window.width() / 2.0 + char_size / 2.0;
    let start_y = window.height() / 2.0 - char_size / 2.0;

    // Load the font
    let font = asset_server.load("fonts/NotoSansJP-VariableFont_wght.ttf");

    // Spawn player body
    commands.spawn((
        create_text_color_bundle(
            font.clone(),
            "@",
            start_x,
            start_y,
            1.0,
            Color::srgb(0.8, 0.8, 0.8),
        ),
        Visibility::default(),
        Player,
        PlayerBody,
        PlayerStats::default(),
    ));

    // Spawn player weapon
    commands.spawn((
        create_text_color_bundle(
            font.clone(),
            "/",
            start_x + char_size,
            start_y,
            1.0,
            Color::srgb(0.8, 0.8, 0.8),
        ),
        Visibility::default(),
        Player,
        PlayerWeapon,
    ));
}

pub fn create_text_color_bundle(
    font: Handle<Font>,
    text: &str,
    x: f32,
    y: f32,
    z: f32,
    color: Color,
) -> (Text2d, TextFont, Transform, TextColor) {
    (
        Text2d::new(text),
        TextFont {
            font,
            font_size: 12.0,
            ..default()
        },
        Transform::from_xyz(x, y, z),
        TextColor::from(color),
    )
}

fn spawn_monsters(
    commands: &mut Commands,
    font: &Handle<Font>,
    start_x: f32,
    start_y: f32,
    spacing: f32,
) {
    // Spawn monsters
    // Oni
    commands.spawn((
        create_text_color_bundle(
            font.clone(),
            "鬼",
            start_x + spacing * 5.0,
            start_y - spacing * 3.0,
            1.0,
            Color::srgb(1.0, 0.0, 0.0),
        ),
        Monster {
            hp: 20,
            max_hp: 20,
            strength: 5,
            name: String::from("Oni"),
            is_alive: true,
        },
    ));

    // Goblin
    commands.spawn((
        create_text_color_bundle(
            font.clone(),
            "G",
            start_x + spacing * 7.0,
            start_y - spacing * 3.0,
            1.0,
            Color::srgb(0.0, 1.0, 0.0),
        ),
        Monster {
            hp: 10,
            max_hp: 10,
            strength: 3,
            name: String::from("Goblin"),
            is_alive: true,
        },
    ));

    // Kappa
    commands.spawn((
        create_text_color_bundle(
            font.clone(),
            "河",
            start_x + spacing * 5.0,
            start_y - spacing * 5.0,
            1.0,
            Color::srgb(0.0, 0.0, 1.0),
        ),
        Monster {
            hp: 15,
            max_hp: 15,
            strength: 4,
            name: String::from("Kappa"),
            is_alive: true,
        },
    ));
}

fn spawn_combat_message(
    commands: &mut Commands,
    font: &Handle<Font>,
    height: f32,
) {
    commands.spawn((
        create_text_color_bundle(
            font.clone(),
            "",
            0.0,
            -height / 2.0 + 12.0,
            2.0,
            Color::srgb(0.8, 0.8, 0.8),
        ),
        CombatMessage {
            message: String::new(),
        },
    ));
}

fn spawn_terrain(
    commands: &mut Commands,
    font: &Handle<Font>,
    width: f32,
    height: f32,
    spacing: f32,
) {
    let cols = (width / spacing) as i32;
    let rows = (height / spacing) as i32;

    let start_x = -width / 2.0 + spacing / 2.0;
    let start_y = height / 2.0 - spacing / 2.0;

    let mut rng = rand::thread_rng();

    for row in 0..rows {
        for col in 0..cols {
            let x = start_x + col as f32 * spacing;
            let y = start_y - row as f32 * spacing;

            // Randomly select terrain type
            let terrain = match rng.gen_range(0..100) {
                0..=60 => crate::terrain::grass(), // 60% chance of grass
                61..=80 => crate::terrain::tree(), // 20% chance of trees
                _ => crate::terrain::rock(),       // 20% chance of rocks
            };

            // Convert terrain to map item
            let map_item = terrain.to_map_item();

            // Spawn the terrain entity
            commands.spawn((
                create_text_color_bundle(
                    font.clone(),
                    map_item.current_character(),
                    x,
                    y,
                    0.0,
                    map_item.current_color(),
                ),
                map_item,
                TerrainEntity,
            ));
        }
    }
}
