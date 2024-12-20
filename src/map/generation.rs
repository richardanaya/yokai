use super::terrain::*;
use bevy::prelude::*;
use noise::{NoiseFn, Perlin};
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

    let perlin = Perlin::new(rng.gen());
    let scale = 0.1; // Adjust this to change the "zoom level" of the noise

    // Create terrain grid
    for row in 0..rows {
        for col in 0..cols {
            let x = start_x + col as f32 * spacing;
            let y = start_y - row as f32 * spacing;

            // Generate noise value for this position
            let noise_val = perlin.get([col as f64 * scale, row as f64 * scale]);
            // Normalize noise from [-1, 1] to [0, 1]
            let normalized_noise = (noise_val + 1.0) / 2.0;

            // Use noise value to determine terrain type
            let terrain = match normalized_noise {
                n if n < 0.3 => earth(),    // Clearings/paths (30%)
                n if n < 0.6 => grass(),    // Forest floor (30%)
                n if n < 0.9 => tree(),     // Dense forest (30%)
                _ => rock(),                // Rocky outcrops (10%)
            };

            // Add some randomness for variation
            if rng.gen_range(0.0..1.0) < 0.1 {  // 10% chance to override
                let _terrain = match rng.gen_range(0..100) {
                    0..=50 => grass(),
                    51..=80 => tree(),
                    _ => rock(),
                };
            }

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
