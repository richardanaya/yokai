use bevy::{
    prelude::*,
    window::{PrimaryWindow, WindowResolution},
};
use rand::seq::SliceRandom;
mod components;
mod map;
mod systems;

use components::*;
use systems::*;

fn main() {
    App::new()
        .add_systems(Startup, setup_audio)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resizable: false,
                resolution: WindowResolution::new(800.0, 600.0),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::srgb(0.2, 0.1, 0.05)))
        .init_state::<GameState>()
        .add_systems(Startup, setup_intro)
        .add_systems(OnEnter(GameState::Playing), (setup, spawn_player))
        .add_systems(Update, spawn_monsters)
        .add_systems(
            Update,
            (
                handle_intro.run_if(in_state(GameState::Intro)),
                fade_out_intro_music
                    .run_if(in_state(GameState::Playing))
                    .into_configs(),
            ),
        )
        .add_systems(
            Update,
            (
                player_movement,
                toggle_inventory,
                render_inventory.run_if(|state: Option<Res<InventoryState>>| state.is_some()),
                cleanup_dead_monsters,
            )
                .chain(),
        )
        .run();
}

fn fade_out_intro_music(audio_state: Res<AudioState>, mut ran: Local<bool>) {
    if !*ran {
        audio_state.into_inner().fade_out(2.0);
        *ran = true;
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    Intro,
    Playing,
}

#[derive(Resource)]
pub struct InventoryState {
    pub needs_update: bool,
}

#[derive(Component)]
pub struct InventoryUI;

#[derive(Component)]
pub struct TerrainEntity;

#[derive(Component)]
pub struct MainCamera;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    // Camera with explicit order
    commands.spawn((
        Camera2d::default(),
        Camera {
            order: 1,
            ..default()
        },
        MainCamera,
    ));

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

    // Generate terrain first
    map::generation::generate_terrain(&mut commands, font.clone(), width, height, char_size);

    // We'll spawn monsters in the next frame when terrain is ready
    commands.spawn_empty().insert(SpawnMonstersMarker);

    // Combat message will be spawned later when needed

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
