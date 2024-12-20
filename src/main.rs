use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    // Text
    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "妖",
            TextStyle {
                // On macOS, use a system Japanese font
                font: Font::try_from_system_path("/System/Library/Fonts/ヒラギノ角ゴシック W3.ttc")
                    .expect("Failed to load system font"),
                font_size: 60.0,
                color: Color::WHITE,
            },
        ),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
}
