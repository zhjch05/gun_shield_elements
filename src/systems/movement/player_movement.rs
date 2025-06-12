use bevy::prelude::*;
use crate::components::{Player, Speed, DirectionIndicator};

/// System to handle player movement with WASD keys
pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<(&mut Transform, &Speed), With<Player>>,
) {
    for (mut transform, speed) in &mut player_query {
        let mut direction = Vec3::ZERO;
        
        // Handle WASD input
        if keyboard_input.pressed(KeyCode::KeyW) {
            direction.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            direction.y -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
        }
        
        // Normalize diagonal movement
        if direction.length() > 0.0 {
            direction = direction.normalize();
        }
        
        // Apply movement
        transform.translation += direction * speed.value * time.delta_secs();
    }
}

/// System to handle player rotation to face mouse cursor and update direction indicator
pub fn player_face_mouse(
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    mut player_query: Query<&mut Transform, (With<Player>, Without<DirectionIndicator>)>,
    mut indicator_query: Query<&mut Transform, (With<DirectionIndicator>, Without<Player>)>,
) {
    if let Ok(window) = windows.single() {
        if let Ok((camera, camera_transform)) = camera_query.single() {
            if let Some(cursor_position) = window.cursor_position() {
                // Convert screen coordinates to world coordinates
                if let Ok(world_position) = camera.viewport_to_world_2d(camera_transform, cursor_position) {
                    // Update player rotation
                    if let Ok(mut player_transform) = player_query.single_mut() {
                        let player_position = player_transform.translation.truncate();
                        let direction = (world_position - player_position).normalize();
                        let angle = direction.y.atan2(direction.x);
                        player_transform.rotation = Quat::from_rotation_z(angle);
                        
                        // Update direction indicator position relative to player
                        if let Ok(mut indicator_transform) = indicator_query.single_mut() {
                            // Position the indicator 30 pixels away from player center in the facing direction
                            let indicator_offset = direction * 32.0;
                            indicator_transform.translation = Vec3::new(
                                player_position.x + indicator_offset.x,
                                player_position.y + indicator_offset.y,
                                1.1 // Slightly above player
                            );
                        }
                    }
                }
            }
        }
    }
}
