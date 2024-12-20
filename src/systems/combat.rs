use crate::components::*;
use bevy::prelude::*;

pub fn cleanup_dead_monsters(mut commands: Commands, query: Query<(Entity, &Monster)>) {
    for (entity, monster) in query.iter() {
        if !monster.is_alive {
            commands.entity(entity).despawn();
        }
    }
}
