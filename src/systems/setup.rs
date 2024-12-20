use crate::{components::*, create_text_color_bundle};
use bevy::{prelude::*, window::PrimaryWindow};

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
