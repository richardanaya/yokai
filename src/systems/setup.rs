use crate::{components::*, create_text_color_bundle, TerrainEntity};
use bevy::{prelude::*, window::PrimaryWindow};

use rand::seq::SliceRandom;

fn find_valid_spawn_position(
    window: &Window,
    char_size: f32,
    terrain_query: &Query<(&Transform, &MapItem), With<TerrainEntity>>,
) -> Option<(f32, f32)> {
    let start_x = -window.width() / 2.0 + char_size / 2.0;
    let start_y = window.height() / 2.0 - char_size / 2.0;
    let mut valid_positions = Vec::new();

    // Collect all valid positions
    for row in 0..((window.height() / char_size) as i32) {
        for col in 0..((window.width() / char_size) as i32) {
            let x = start_x + col as f32 * char_size;
            let y = start_y - row as f32 * char_size;
            
            let mut is_valid = true;
            for (transform, map_item) in terrain_query.iter() {
                if (transform.translation.x - x).abs() < 1.0 && 
                   (transform.translation.y - y).abs() < 1.0 {
                    if map_item.solid {
                        is_valid = false;
                        break;
                    }
                }
            }
            
            if is_valid {
                valid_positions.push((x, y));
            }
        }
    }

    // Randomly select one of the valid positions
    valid_positions.choose(&mut rand::thread_rng()).copied()
}

pub fn spawn_monsters(
    mut commands: Commands,
    marker_query: Query<Entity, With<SpawnMonstersMarker>>,
    terrain_query: Query<(&Transform, &MapItem), With<TerrainEntity>>,
    asset_server: Res<AssetServer>,
) {
    // Only run if we have the marker
    if marker_query.is_empty() {
        return;
    }

    let font = asset_server.load("fonts/NotoSansJP-VariableFont_wght.ttf");
    let mut valid_positions = Vec::new();

    // Collect all non-solid positions
    for (transform, map_item) in terrain_query.iter() {
        if !map_item.solid {
            valid_positions.push((transform.translation.x, transform.translation.y));
        }
    }

    // Spawn monsters at random valid positions
    if !valid_positions.is_empty() {
        use rand::seq::SliceRandom;
        let mut rng = rand::thread_rng();

        // Oni
        if let Some(pos) = valid_positions.choose(&mut rng) {
            commands.spawn((
                create_text_color_bundle(
                    font.clone(),
                    "鬼",
                    pos.0,
                    pos.1,
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
        }

        // Goblin
        if let Some(pos) = valid_positions.choose(&mut rng) {
            commands.spawn((
                create_text_color_bundle(
                    font.clone(),
                    "G",
                    pos.0,
                    pos.1,
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
        }

        // Kappa
        if let Some(pos) = valid_positions.choose(&mut rng) {
            commands.spawn((
                create_text_color_bundle(
                    font.clone(),
                    "河",
                    pos.0,
                    pos.1,
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
    }

    // Clean up the marker
    for entity in marker_query.iter() {
        commands.entity(entity).despawn();
    }
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
