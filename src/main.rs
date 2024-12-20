use bevy::{
    input::keyboard::KeyCode,
    prelude::*,
    window::{PrimaryWindow, WindowResolution},
};
mod components;
mod terrain;
use components::*;
use terrain::*;

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
            ),
        )
        .run();
}

fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let _window = window_query.single();
    // Load the font
    let font = asset_server.load("fonts/NotoSansJP-VariableFont_wght.ttf");

    let window = window_query.single();
    let char_size = 12.0;
    // Calculate starting position (top-left corner)
    let start_x = -window.width() / 2.0 + char_size / 2.0;
    let start_y = window.height() / 2.0 - char_size / 2.0;

    // Spawn player body
    commands.spawn((
        create_text_color_bundle(font.clone(), "@", start_x, start_y, 1.0, Color::srgb(0.8, 0.8, 0.8)),
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

fn toggle_inventory(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut PlayerStats, With<Player>>,
    mut commands: Commands,
    inventory_ui: Query<Entity, With<InventoryUI>>,
    terrain_entities: Query<Entity, With<TerrainEntity>>,
    mut player_visibility: Query<&mut Visibility, With<Player>>, 
    camera_query: Query<Entity, With<MainCamera>>,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if keyboard.just_pressed(KeyCode::KeyI) {
        if let Ok(mut stats) = query.get_single_mut() {
            stats.show_inventory = !stats.show_inventory;
            
            // Clean up existing UI elements
            for entity in inventory_ui.iter() {
                commands.entity(entity).despawn();
            }
            
            if stats.show_inventory {
                // Hide terrain when showing inventory
                for entity in terrain_entities.iter() {
                    commands.entity(entity).despawn();
                }
                
                // Hide player
                for mut visibility in player_visibility.iter_mut() {
                    *visibility = Visibility::Hidden;
                }
                
                commands.insert_resource(InventoryState { needs_update: true });
                
                // Setup inventory display
                setup_inventory_display(&mut commands, &asset_server, &window_query);
            } else {
                // Show player
                for mut visibility in player_visibility.iter_mut() {
                    *visibility = Visibility::Inherited;
                }
                
                commands.remove_resource::<InventoryState>();
                
                // Remove existing camera
                if let Ok(camera_entity) = camera_query.get_single() {
                    commands.entity(camera_entity).despawn();
                }
                
                // Recreate terrain and camera
                setup(commands, asset_server, window_query);
            }
        }
    }
}

fn setup_inventory_display(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    window_query: &Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single();
    let font = asset_server.load("fonts/NotoSansJP-VariableFont_wght.ttf");
    
    // Create a dark background for the inventory
    for row in 0..50 {
        for col in 0..100 {
            let x = -window.width() / 2.0 + col as f32 * 12.0;
            let y = window.height() / 2.0 - row as f32 * 12.0;
            
            commands.spawn((
                create_text_color_bundle(
                    font.clone(),
                    ".",
                    x,
                    y,
                    0.0,
                    Color::srgb(0.1, 0.1, 0.1),
                ),
                InventoryUI,
            ));
        }
    }
}

fn player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    let grid_size = 12.0;
    let mut delta = Vec2::ZERO;

    if keyboard.just_pressed(KeyCode::KeyW) {
        delta.y += grid_size;
    }
    if keyboard.just_pressed(KeyCode::KeyS) {
        delta.y -= grid_size;
    }
    if keyboard.just_pressed(KeyCode::KeyA) {
        delta.x -= grid_size;
    }
    if keyboard.just_pressed(KeyCode::KeyD) {
        delta.x += grid_size;
    }

    if delta != Vec2::ZERO {
        for mut transform in player_query.iter_mut() {
            transform.translation.x += delta.x;
            transform.translation.y += delta.y;
        }
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    // Camera
    commands.spawn((Camera2d::default(), MainCamera));

    // Load the font
    let font = asset_server.load("fonts/NotoSansJP-VariableFont_wght.ttf");

    // Spawn monster
    commands.spawn((
        create_text_color_bundle(
            font.clone(),
            "鬼",
            start_x + spacing * 5.0,  // 5 spaces right of origin
            start_y - spacing * 3.0,  // 3 spaces down
            1.0,
            Color::RED,
        ),
        Monster {
            hp: 20,
            max_hp: 20,
            strength: 5,
        },
    ));

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
