use bevy::{
    input::keyboard::KeyCode,
    prelude::*,
    window::{PrimaryWindow, WindowResolution},
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
        .add_systems(Update, player_movement)
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

    // Create terrain with random distribution
    for row in 0..game_map.height {
        for col in 0..game_map.width {
            // Randomly select terrain type
            let terrain = match rng.gen_range(0..100) {
                0..=60 => grass(), // 60% chance of grass
                61..=80 => tree(), // 20% chance of trees
                _ => rock(),       // 20% chance of rocks
            };

            let item = commands
                .spawn((
                    terrain.to_map_item(),
                    MapPosition {
                        x: col,
                        y: row,
                        z: 0,
                    },
                ))
                .id();
            game_map.add_item(col, row, item);
        }
    }

    for row in 0..rows {
        for col in 0..cols {
            let x = start_x + col as f32 * spacing;
            let y = start_y - row as f32 * spacing;

            // For now, just show the ground layer
            commands.spawn(create_text_color_bundle(
                font.clone(),
                "地",
                x,
                y,
                0.0,
                Color::srgb(0.2, 0.5, 0.2),
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
