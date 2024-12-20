use bevy::{prelude::*, window::PrimaryWindow};
use crate::{
    components::*,
    TerrainEntity,
    MainCamera,
    InventoryUI,
    InventoryState,
    create_text_color_bundle,
};

pub fn toggle_inventory(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut PlayerStats, With<Player>>,
    mut commands: Commands,
    terrain_entities: Query<Entity, With<TerrainEntity>>,
    mut player_visibility: Query<&mut Visibility, With<Player>>,
    camera_query: Query<Entity, With<MainCamera>>,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    inventory_ui: Query<Entity, With<InventoryUI>>,
) {
    if keyboard.just_pressed(KeyCode::KeyI) {
        if let Ok(mut stats) = query.get_single_mut() {
            stats.show_inventory = !stats.show_inventory;

            if stats.show_inventory {
                commands.insert_resource(InventoryState { needs_update: true });
                setup_inventory_display(&mut commands, &asset_server, &window_query);
            } else {
                // Clean up inventory UI when toggling off
                for entity in inventory_ui.iter() {
                    commands.entity(entity).despawn();
                }
                commands.remove_resource::<InventoryState>();
            }
        }
    }
}

pub fn setup_inventory_display(
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
                create_text_color_bundle(font.clone(), ".", x, y, 0.0, Color::srgb(0.1, 0.1, 0.1)),
                InventoryUI,
            ));
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
    // Inventory rendering logic here
}
