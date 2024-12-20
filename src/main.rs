use bevy::{prelude::*, window::{WindowResolution, PrimaryWindow}};
mod components;
use components::*;

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
        .insert_resource(GameMap::new(100, 100))  // Create a 100x100 map
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    #[allow(clippy::type_complexity)]
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut game_map: ResMut<GameMap>,
) {
    // Camera
    commands.spawn(Camera2dBundle::default());

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

    // Create some example terrain
    for row in 0..game_map.height {
        for col in 0..game_map.width {
            // Create ground layer
            let ground = commands.spawn((
                MapItem {
                    character: "地".to_string(),
                    color: Color::rgb(0.2, 0.5, 0.2),
                },
                MapPosition { x: col, y: row, z: 0 },
            )).id();
            game_map.add_item(col, row, ground);

            // Add some random features
            if (col + row) % 7 == 0 {
                let tree = commands.spawn((
                    MapItem {
                        character: "木".to_string(),
                        color: Color::rgb(0.0, 0.8, 0.0),
                    },
                    MapPosition { x: col, y: row, z: 1 },
                )).id();
                game_map.add_item(col, row, tree);
            }
        }
    }

    // Spawn visible grid of characters (viewport)
    for row in 0..rows {
        for col in 0..cols {
            let x = start_x + col as f32 * spacing;
            let y = start_y - row as f32 * spacing;

            // For now, just show the ground layer
            commands.spawn(Text2dBundle {
                text: Text::from_section(
                    "地",
                    TextStyle {
                        font: font.clone(),
                        font_size: char_size,
                        color: Color::rgb(0.2, 0.5, 0.2),
                    },
                ),
                transform: Transform::from_xyz(x, y, 0.0),
                ..default()
            });
        }
    }
}
