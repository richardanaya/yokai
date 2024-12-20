use bevy::prelude::*;
use bevy::window::PrimaryWindow;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
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

    // Calculate grid dimensions
    let cols = (width / spacing) as i32;
    let rows = (height / spacing) as i32;

    // Calculate starting position (top-left corner)
    let start_x = -width / 2.0 + spacing / 2.0;
    let start_y = height / 2.0 - spacing / 2.0;

    // Spawn grid of characters
    for row in 0..rows {
        for col in 0..cols {
            let x = start_x + col as f32 * spacing;
            let y = start_y - row as f32 * spacing;

            commands.spawn(Text2dBundle {
                text: Text::from_section(
                    "妖",
                    TextStyle {
                        font: font.clone(),
                        font_size: char_size,
                        color: Color::WHITE,
                    },
                ),
                transform: Transform::from_xyz(x, y, 0.0),
                ..default()
            });
        }
    }
}
