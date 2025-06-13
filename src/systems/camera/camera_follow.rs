use bevy::prelude::*;
use crate::components::{Player, PlayerDash};

/// System to make camera follow the player
pub fn camera_follow_player(
    player_query: Query<(&Transform, &PlayerDash), (With<Player>, Without<Camera2d>)>,
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
    time: Res<Time>,
) {
    if let Ok((player_transform, player_dash)) = player_query.single() {
        if let Ok(mut camera_transform) = camera_query.single_mut() {
            let target_position = player_transform.translation;
            
            // Adjust follow speed based on whether player is dashing
            let follow_speed = if player_dash.is_dashing {
                // During dash, follow faster to keep up
                15.0 // Higher follow speed during dash
            } else {
                // Regular movement, smoother following
                8.0 // Regular follow speed
            };
            
            // Use time-based lerp for consistent behavior regardless of framerate
            let lerp_factor = (follow_speed * time.delta_secs()).min(1.0);
            camera_transform.translation = camera_transform.translation.lerp(target_position, lerp_factor);
        }
    }
}
