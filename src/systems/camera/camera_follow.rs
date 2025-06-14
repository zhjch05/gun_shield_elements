use bevy::prelude::*;
use crate::components::{Player};
use crate::constants::GameBoundaries;

/// System to make camera follow the player while respecting boundaries
pub fn camera_follow_player(
    player_query: Query<&Transform, (With<Player>, Without<Camera2d>)>,
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
    windows: Query<&Window>,
) {
    if let Ok(player_transform) = player_query.single() {
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
                
                // Set camera position directly to target for exact following
                camera_transform.translation = clamped_target;
            } else {
                // Fallback: set camera position directly to player position
                camera_transform.translation = target_position;
            }
        }
    }
}
