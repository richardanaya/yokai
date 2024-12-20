use crate::{components::*, create_text_color_bundle, TerrainEntity};
use bevy::{prelude::*, window::PrimaryWindow};
use crate::map::terrain::TerrainType;

fn find_valid_spawn_position(
    window: &Window,
    char_size: f32,
    terrain_query: &Query<(&Transform, &MapItem), With<TerrainEntity>>,
) -> Option<(f32, f32)> {
    let start_x = -window.width() / 2.0 + char_size / 2.0;
    let start_y = window.height() / 2.0 - char_size / 2.0;

    // Try positions until we find one that's not solid
    for row in 0..((window.height() / char_size) as i32) {
        for col in 0..((window.width() / char_size) as i32) {
            let x = start_x + col as f32 * char_size;
            let y = start_y - row as f32 * char_size;
            
            let mut is_valid = true;
            for (transform, _) in terrain_query.iter() {
                if (transform.translation.x - x).abs() < 1.0 && 
                   (transform.translation.y - y).abs() < 1.0 {
                    // Found terrain at this position, check if it's solid
                    is_valid = false;
                    break;
                }
            }
            
            if is_valid {
                return Some((x, y));
            }
        }
    }
    None
}

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    terrain_query: Query<(&Transform, &MapItem), With<TerrainEntity>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.single();
    let char_size = 12.0;

    // Find a valid spawn position
    let (start_x, start_y) = match find_valid_spawn_position(window, char_size, &terrain_query) {
        Some(pos) => pos,
        None => {
            // Fallback to default position if no valid position found
            let x = -window.width() / 2.0 + char_size / 2.0;
            let y = window.height() / 2.0 - char_size / 2.0;
            (x, y)
        }
    };

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
