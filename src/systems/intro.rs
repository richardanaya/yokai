use crate::GameState;
use bevy::{input::keyboard::KeyboardInput, prelude::*, text::FontSmoothing};

#[derive(Component)]
pub struct IntroCamera;

#[derive(Component, Default)]
pub struct IntroText;

#[derive(Bundle, Default)]
pub struct IntroTextBundle {
    pub text: Text2d,
    pub text_font: TextFont,
    pub text_color: TextColor,
    pub transform: Transform,
    pub intro: IntroText,
}

pub fn setup_intro(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera with explicit order
    commands.spawn((
        Camera2d::default(),
        Camera {
            order: 0,
            ..default()
        },
        IntroCamera,
    ));

    // Load the font
    let font = asset_server.load("fonts/NotoSansJP-VariableFont_wght.ttf");

    // Spawn title text
    commands.spawn(IntroTextBundle {
        text: Text2d::new("妖怪"),
        text_font: TextFont {
            font: font.clone(),
            font_size: 60.0,
            font_smoothing: bevy::text::FontSmoothing::AntiAliased,
        },
        text_color: TextColor::from(Color::srgb(1.0, 1.0, 1.0)),
        transform: Transform::from_xyz(0.0, 50.0, 0.0),
        ..default()
    });

    // Spawn "Press any key" text
    commands.spawn(IntroTextBundle {
        text: Text2d::new("Press any key to start"),
        text_font: TextFont {
            font,
            font_size: 20.0,
            font_smoothing: FontSmoothing::AntiAliased,
        },
        text_color: TextColor::from(Color::srgb(0.5, 0.5, 0.5)),
        transform: Transform::from_xyz(0.0, -50.0, 0.0),
        ..default()
    });
}

pub fn handle_intro(
    mut next_state: ResMut<NextState<GameState>>,
    mut keyboard_events: EventReader<KeyboardInput>,
    mut commands: Commands,
    intro_text: Query<Entity, With<IntroText>>,
    intro_camera: Query<Entity, With<IntroCamera>>,
) {
    for _ in keyboard_events.read() {
        // Clean up intro text and camera
        for entity in intro_text.iter().chain(intro_camera.iter()) {
            commands.entity(entity).despawn();
        }
        next_state.set(GameState::Playing);
        break;
    }
}
