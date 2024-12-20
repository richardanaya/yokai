use bevy::{
    input::keyboard::KeyCode,
    prelude::*,
    window::{PrimaryWindow, WindowResolution},
};
mod components;
mod terrain;
mod systems;

use components::*;
use terrain::*;
use systems::*;

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
        .add_systems(Startup, (setup, spawn_player))
        .add_systems(
            Update,
            (
                player_movement,
                toggle_inventory,
                render_inventory.run_if(|state: Option<Res<InventoryState>>| state.is_some()),
                cleanup_dead_monsters,
            ),
        )
        .run();
}


#[derive(Resource)]
struct InventoryState {
    needs_update: bool,
}

#[derive(Component)]
struct InventoryUI;

#[derive(Component)]
struct TerrainEntity;

#[derive(Component)]
struct MainCamera;



fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    // Camera
    commands.spawn((Camera2d::default(), MainCamera));

    // Load the font
    let font = asset_server.load("fonts/NotoSansJP-VariableFont_wght.ttf");

    // Get window dimensions
    let window = window_query.single();
    let width = window.width();
    let height = window.height();

    // Character size and spacing
    let char_size = 12.0;
    let spacing = char_size;

    // Calculate starting position (top-left corner)
    let start_x = -width / 2.0 + spacing / 2.0;
    let start_y = height / 2.0 - spacing / 2.0;

    // Spawn monsters
    // Oni
    commands.spawn((
        create_text_color_bundle(
            font.clone(),
            "鬼",
            start_x + spacing * 5.0,
            start_y - spacing * 3.0,
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

    // Goblin
    commands.spawn((
        create_text_color_bundle(
            font.clone(),
            "G",
            start_x + spacing * 7.0,
            start_y - spacing * 3.0,
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

    // Kappa
    commands.spawn((
        create_text_color_bundle(
            font.clone(),
            "河",
            start_x + spacing * 5.0,
            start_y - spacing * 5.0,
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

    // Spawn combat message bar
    commands.spawn((
        create_text_color_bundle(
            font.clone(),
            "",
            0.0,                  // Left edge + small margin
            -height / 2.0 + 12.0, // Bottom edge + small margin
            2.0,
            Color::srgb(0.8, 0.8, 0.8),
        ),
        CombatMessage {
            message: String::new(),
        },
    ));
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

    // Create terrain grid
    for row in 0..rows {
        for col in 0..cols {
            let x = start_x + col as f32 * spacing;
            let y = start_y - row as f32 * spacing;

            // Randomly select terrain type
            let terrain = match rng.gen_range(0..100) {
                0..=60 => grass(), // 60% chance of grass
                61..=80 => tree(), // 20% chance of trees
                _ => rock(),       // 20% chance of rocks
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

fn cleanup_inventory_ui(
    mut commands: Commands,
    inventory_ui: Query<Entity, With<InventoryUI>>,
) {
    for entity in inventory_ui.iter() {
        commands.entity(entity).despawn();
    }
}

fn cleanup_dead_monsters(
    mut commands: Commands,
    query: Query<(Entity, &Monster)>,
) {
    for (entity, monster) in query.iter() {
        if !monster.is_alive {
            commands.entity(entity).despawn();
        }
    }
}

fn render_inventory(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<&PlayerStats, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut inventory_state: ResMut<InventoryState>,
) {
    if !inventory_state.needs_update {
        return;
    }

    if let Ok(stats) = query.get_single() {
        if stats.show_inventory {
            inventory_state.needs_update = false;
            let window = window_query.single();
            let font = asset_server.load("fonts/NotoSansJP-VariableFont_wght.ttf");

            // Create inventory overlay
            let overlay = format!(
                "╔═══ Character Stats ═══╗\n\
                 ║ Level: {:13} ║\n\
                 ║ EXP: {}/100         ║\n\
                 ║ HP: {}/{}           ║\n\
                 ║ MP: {}/{}           ║\n\
                 ╟──── Attributes ─────╢\n\
                 ║ STR: {:13} ║\n\
                 ║ DEX: {:13} ║\n\
                 ║ CON: {:13} ║\n\
                 ║ INT: {:13} ║\n\
                 ║ WIS: {:13} ║\n\
                 ║ CHA: {:13} ║\n\
                 ╚═══════════════════════╝",
                stats.level,
                stats.exp,
                stats.hp,
                stats.max_hp,
                stats.mp,
                stats.max_mp,
                stats.strength,
                stats.dexterity,
                stats.constitution,
                stats.intelligence,
                stats.wisdom,
                stats.charisma
            );

            commands.spawn((
                create_text_color_bundle(
                    font,
                    &overlay,
                    -window.width() / 2.0 + 150.0,
                    window.height() / 2.0 - 100.0,
                    0.0,
                    Color::srgb(0.8, 0.8, 0.8),
                ),
                InventoryUI,
            ));
        }
    }
}
