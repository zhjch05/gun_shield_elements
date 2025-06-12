use bevy::prelude::*;
use crate::components::Player;

/// System to make camera follow the player
pub fn camera_follow_player(
    player_query: Query<&Transform, (With<Player>, Without<Camera2d>)>,
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
) {
    if let Ok(player_transform) = player_query.single() {
        if let Ok(mut camera_transform) = camera_query.single_mut() {
            // Smoothly follow the player
            let target_position = player_transform.translation;
            camera_transform.translation = camera_transform.translation.lerp(target_position, 0.1);
        }
    }
}
