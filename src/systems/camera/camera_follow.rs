use bevy::prelude::*;
use crate::components::{Player, PlayerDash};
use crate::constants::GameBoundaries;

/// System to make camera follow the player while respecting boundaries
pub fn camera_follow_player(
    player_query: Query<(&Transform, &PlayerDash), (With<Player>, Without<Camera2d>)>,
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
    time: Res<Time>,
    windows: Query<&Window>,
) {
    if let Ok((player_transform, player_dash)) = player_query.single() {
        if let Ok(mut camera_transform) = camera_query.single_mut() {
            let target_position = player_transform.translation;
            
            // Get viewport size for boundary calculations
            if let Ok(window) = windows.single() {
                let viewport_width = window.width();
                let viewport_height = window.height();
                
                // Calculate camera boundaries based on world boundaries and viewport size
                let camera_half_width = viewport_width / 2.0;
                let camera_half_height = viewport_height / 2.0;
                
                // Clamp camera position to ensure it doesn't show areas outside the game world
                let clamped_target = Vec3::new(
                    target_position.x.clamp(
                        GameBoundaries::min_x() + camera_half_width,
                        GameBoundaries::max_x() - camera_half_width
                    ),
                    target_position.y.clamp(
                        GameBoundaries::min_y() + camera_half_height,
                        GameBoundaries::max_y() - camera_half_height
                    ),
                    target_position.z
                );
                
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
                camera_transform.translation = camera_transform.translation.lerp(clamped_target, lerp_factor);
            } else {
                // Fallback to original behavior if window query fails
                let follow_speed = if player_dash.is_dashing {
                    15.0
                } else {
                    8.0
                };
                
                let lerp_factor = (follow_speed * time.delta_secs()).min(1.0);
                camera_transform.translation = camera_transform.translation.lerp(target_position, lerp_factor);
            }
        }
    }
}
