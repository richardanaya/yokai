use crate::components::*;
use bevy::prelude::*;

pub fn player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut param_set: ParamSet<(
        Query<&mut Transform, With<Player>>,
        Query<(&Transform, &mut Monster)>,
    )>,
    mut message_query: Query<(&mut Text2d, &mut CombatMessage)>,
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
        let player_pos = param_set.p0().iter().next().unwrap().translation.clone();
        let new_pos = Vec3::new(player_pos.x + delta.x, player_pos.y + delta.y, player_pos.z);

        // Check for monster collision
        let mut collided = false;
        for (monster_transform, mut monster) in param_set.p1().iter_mut() {
            if (monster_transform.translation.x - new_pos.x).abs() < 1.0
                && (monster_transform.translation.y - new_pos.y).abs() < 1.0
                && monster.is_alive
            {
                collided = true;
                // Combat logic
                monster.hp = monster.hp.saturating_sub(5); // Player deals 5 damage

                if monster.hp == 0 {
                    monster.is_alive = false;
                    if let Ok((mut text, mut message)) = message_query.get_single_mut() {
                        message.message = format!("You defeated the {}!", monster.name);
                        text.0 = message.message.clone();
                    }
                } else {
                    if let Ok((mut text, mut message)) = message_query.get_single_mut() {
                        message.message =
                            format!("You hit the {}! ({} HP left)", monster.name, monster.hp);
                        text.0 = message.message.clone();
                    }
                }
                break;
            }
        }

        if !collided {
            // Move both player body and weapon
            for mut transform in param_set.p0().iter_mut() {
                if transform.translation.x == player_pos.x {
                    // This is the body
                    transform.translation = new_pos;
                } else {
                    // This is the weapon
                    transform.translation = Vec3::new(new_pos.x + 12.0, new_pos.y, new_pos.z);
                }
            }
        }
    }
}
