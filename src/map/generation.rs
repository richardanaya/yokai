use super::terrain::*;
use bevy::prelude::*;
use rand::Rng;
use crate::TerrainEntity;
use crate::create_text_color_bundle;

pub fn generate_terrain(
    commands: &mut Commands,
    font: Handle<Font>,
    width: f32,
    height: f32,
    char_size: f32,
) {
    let spacing = char_size;

    // Calculate visible grid dimensions
    let cols = (width / spacing) as i32;
    let rows = (height / spacing) as i32;

    // Calculate starting position (top-left corner)
    let start_x = -width / 2.0 + spacing / 2.0;
    let start_y = height / 2.0 - spacing / 2.0;

    let mut rng = rand::thread_rng();

    // Create terrain grid
    for row in 0..rows {
        for col in 0..cols {
            let x = start_x + col as f32 * spacing;
            let y = start_y - row as f32 * spacing;

            // Randomly select terrain type
            let terrain = match rng.gen_range(0..100) {
                0..=50 => grass(), // 50% chance of grass
                51..=70 => tree(), // 20% chance of trees
                71..=85 => rock(), // 15% chance of rocks
                _ => earth(),      // 15% chance of bare earth
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
