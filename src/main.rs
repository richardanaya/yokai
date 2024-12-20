use bevy::{
    input::keyboard::KeyCode,
    prelude::*,
    window::{PrimaryWindow, WindowResolution},
    text::Text2dBounds,
};
mod components;
mod terrain;
use components::*;
use terrain::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resizable: false,
                resolution: WindowResolution::new(800.0, 600.0),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(GameMap::new(100, 100)) // Create a 100x100 map
        .add_systems(Startup, (setup, spawn_player))
        .add_systems(Update, (player_movement, toggle_inventory, render_inventory))
        .run();
}

fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.single();
    // Load the font
    let font = asset_server.load("fonts/NotoSansJP-VariableFont_wght.ttf");

    // Spawn player body
    commands.spawn((
        create_text_color_bundle(font.clone(), "@", 0.0, 0.0, 1.0, Color::srgb(0.8, 0.8, 0.8)),
        Player,
        PlayerBody {
            character: "@".to_string(),
        },
        PlayerStats::default(),
    ));

    // Spawn player weapon
    commands.spawn((
        create_text_color_bundle(
            font.clone(),
            "/",
            12.0,
            0.0,
            1.0,
            Color::srgb(0.8, 0.8, 0.8),
        ),
        Player,
        PlayerWeapon {
            character: "/".to_string(),
        },
    ));
}

fn toggle_inventory(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut PlayerStats, With<Player>>,
) {
    if keyboard.just_pressed(KeyCode::KeyI) {
        if let Ok(mut stats) = query.get_single_mut() {
            stats.show_inventory = !stats.show_inventory;
        }
    }
}

fn player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    let movement_speed = 5.0;
    let mut direction = Vec2::ZERO;

    if keyboard.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }

    if direction != Vec2::ZERO {
        direction = direction.normalize();
        for mut transform in player_query.iter_mut() {
            transform.translation.x += direction.x * movement_speed;
            transform.translation.y += direction.y * movement_speed;
        }
    }
}

fn setup(
    #[allow(clippy::type_complexity)] mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut game_map: ResMut<GameMap>,
) {
    // Camera
    commands.spawn(Camera2d::default());

    // Load the font
    let font = asset_server.load("fonts/NotoSansJP-VariableFont_wght.ttf");

    // Get window dimensions
    let window = window_query.single();
    let width = window.width();
    let height = window.height();

    // Character size and spacing
    let char_size = 12.0;
    let spacing = char_size;

    // Calculate visible grid dimensions
    let cols = (width / spacing) as i32;
    let rows = (height / spacing) as i32;

    // Calculate starting position (top-left corner)
    let start_x = -width / 2.0 + spacing / 2.0;
    let start_y = height / 2.0 - spacing / 2.0;

    use rand::Rng;
    let mut rng = rand::thread_rng();

    // Create terrain grid
    for row in 0..rows {
        for col in 0..cols {
            let x = start_x + col as f32 * spacing;
            let y = start_y - row as f32 * spacing;

            // Randomly select terrain type
            let terrain = match rng.gen_range(0..100) {
                0..=60 => grass(), // 60% chance of grass
                61..=80 => tree(), // 20% chance of trees
                _ => rock(),       // 20% chance of rocks
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
                MapPosition {
                    x: col as i32,
                    y: row as i32,
                    z: 0,
                },
            ));
        }
    }
}

fn create_text_color_bundle(
    font: Handle<Font>,
    text: &str,
    x: f32,
    y: f32,
    z: f32,
    color: Color,
) -> (Text2d, TextFont, Transform, TextColor) {
    return (
        Text2d::new(text),
        TextFont {
            font: font,
            font_size: 12.0,
            ..default()
        },
        Transform::from_xyz(x, y, z),
        TextColor::from(color),
    );
}

fn render_inventory(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<&PlayerStats, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(stats) = query.get_single() {
        if stats.show_inventory {
            let window = window_query.single();
            let font = asset_server.load("fonts/NotoSansJP-VariableFont_wght.ttf");
            
            // Create inventory overlay
            let overlay = format!(
                "╔═══ Character Stats ═══╗\n\
                 ║ Level: {:13} ║\n\
                 ║ EXP: {}/100         ║\n\
                 ║ HP: {}/{}           ║\n\
                 ║ MP: {}/{}           ║\n\
                 ╟──── Attributes ─────╢\n\
                 ║ STR: {:13} ║\n\
                 ║ DEX: {:13} ║\n\
                 ║ CON: {:13} ║\n\
                 ║ INT: {:13} ║\n\
                 ║ WIS: {:13} ║\n\
                 ║ CHA: {:13} ║\n\
                 ╚═══════════════════════╝",
                stats.level,
                stats.exp,
                stats.hp, stats.max_hp,
                stats.mp, stats.max_mp,
                stats.strength,
                stats.dexterity,
                stats.constitution,
                stats.intelligence,
                stats.wisdom,
                stats.charisma
            );

            commands.spawn((
                Text2dBundle {
                    text: Text::from_section(
                        overlay,
                        TextStyle {
                            font: font.clone(),
                            font_size: 20.0,
                            color: Color::WHITE,
                        },
                    ),
                    transform: Transform::from_xyz(
                        -window.width() / 2.0 + 20.0,
                        window.height() / 2.0 - 20.0,
                        10.0,
                    ),
                    ..default()
                },
            ));
        }
    }
}
