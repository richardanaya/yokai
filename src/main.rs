use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    // Text
    // Load the font
    let font = asset_server.load("fonts/NotoSansJP-VariableFont_wght.ttf");
    
    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "妖",
            TextStyle {
                font,
                font_size: 60.0,
                color: Color::WHITE,
            },
        ),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
}
