use crate::{
    components::*, create_text_color_bundle, InventoryState, InventoryUI, MainCamera, TerrainEntity,
};
use bevy::{prelude::*, window::PrimaryWindow};

pub fn toggle_inventory(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut PlayerStats, With<Player>>,
    mut commands: Commands,
    _terrain_entities: Query<Entity, With<TerrainEntity>>,
    _camera_query: Query<Entity, With<MainCamera>>,
    inventory_ui: Query<Entity, With<InventoryUI>>,
) {
    if keyboard.just_pressed(KeyCode::KeyI) {
        if let Ok(mut stats) = query.get_single_mut() {
            stats.show_inventory = !stats.show_inventory;

            if stats.show_inventory {
                commands.insert_resource(InventoryState { needs_update: true });
            } else {
                // Clean up inventory UI when toggling off
                for entity in inventory_ui.iter() {
                    println!("Despawning inventory UI");
                    commands.entity(entity).despawn();
                }
                commands.remove_resource::<InventoryState>();
            }
        }
    }
}

pub fn render_inventory(
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

            println!("Rendering inventory");
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
