use bevy::prelude::*;

pub fn create_text_color_bundle(
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
