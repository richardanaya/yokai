use crate::GameState;
use bevy::{input::keyboard::KeyboardInput, prelude::*, text::FontSmoothing};
use std::f32::consts::PI;

#[derive(Component)]
pub struct IntroCamera;

#[derive(Component)]
pub struct TitleText;

#[derive(Component)]
pub struct PressKeyText {
    timer: Timer,
}

#[derive(Component)]
pub struct BackgroundText;

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

    // Spawn background decorative kanji
    let background_kanji = vec!["神", "鬼", "龍", "虎", "蛇", "狐", "兎", "鳥"];
    for (i, kanji) in background_kanji.iter().enumerate() {
        let angle = (i as f32 / background_kanji.len() as f32) * 2.0 * PI;
        let radius = 200.0;
        let x = angle.cos() * radius;
        let y = angle.sin() * radius;
        commands.spawn((
            Text2d::new(kanji.to_string()),
            TextFont {
                font: font.clone(),
                font_size: 40.0,
                font_smoothing: FontSmoothing::AntiAliased,
            },
            TextColor::from(Color::srgb(0.2, 0.2, 0.2)),
            Transform::from_xyz(x, y, -1.0),
            BackgroundText,
            IntroText,
        ));
    }

    // Spawn title text
    commands.spawn((
        Text2d::new("妖怪"),
        TextFont {
            font: font.clone(),
            font_size: 100.0,
            font_smoothing: FontSmoothing::AntiAliased,
        },
        TextColor::from(Color::srgb(1.0, 0.8, 0.8)),
        Transform::from_xyz(0.0, 50.0, 1.0),
        TitleText,
        IntroText,
    ));

    // Spawn "Press any key" text
    commands.spawn((
        Text2d::new("Press any key to start"),
        TextFont {
            font,
            font_size: 24.0,
            font_smoothing: FontSmoothing::AntiAliased,
        },
        TextColor::from(Color::srgb(0.7, 0.7, 0.7)),
        Transform::from_xyz(0.0, -50.0, 1.0),
        PressKeyText {
            timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        },
        IntroText,
    ));
}

pub fn handle_intro(
    mut next_state: ResMut<NextState<GameState>>,
    mut keyboard_events: EventReader<KeyboardInput>,
    mut commands: Commands,
    intro_text: Query<Entity, With<IntroText>>,
    intro_camera: Query<Entity, With<IntroCamera>>,
    time: Res<Time>,
    mut title_query: Query<(&mut Transform, &mut TextColor), With<TitleText>>,
    mut press_key_query: Query<(&mut TextColor, &mut PressKeyText), With<PressKeyText>>,
    mut background_query: Query<&mut Transform, With<BackgroundText>>,
) {
    // Animate title pulse
    if let Ok((mut transform, mut color)) = title_query.get_single_mut() {
        let scale = 1.0 + (time.elapsed_secs() * 2.0).sin() * 0.1;
        transform.scale = Vec3::splat(scale);
        let brightness = 0.8 + (time.elapsed_secs() * 3.0).sin() * 0.2;
        color.0 = Color::srgb(1.0, brightness, brightness);
    }

    // Animate press key text fade
    if let Ok((mut color, mut timer)) = press_key_query.get_single_mut() {
        timer.timer.tick(time.delta());
        let alpha = (timer.timer.fraction() * PI).sin().abs();
        color.0.set_a(alpha);
        color.0 = color.0.with_a(alpha);
    }

    // Rotate background kanji
    for mut transform in background_query.iter_mut() {
        transform.rotate_z(time.delta_secs() * 0.2);
    }
    for _ in keyboard_events.read() {
        // Clean up intro text and camera
        for entity in intro_text.iter().chain(intro_camera.iter()) {
            commands.entity(entity).despawn();
        }
        next_state.set(GameState::Playing);
        break;
    }
}
