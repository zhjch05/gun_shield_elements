use bevy::prelude::*;
use crate::components::{Player, DirectionIndicator, PlayerDash, Energy, Invulnerability, Shield};

/// System to handle player movement and dash input
pub fn player_movement(
    mut player_query: Query<(&mut Transform, &mut PlayerDash, &mut Energy), (With<Player>, Without<DirectionIndicator>)>,
    shield_query: Query<&Shield>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (mut player_transform, mut dash, mut energy) in &mut player_query {
        let delta = time.delta_secs();
        
        // Recharge energy only if shield is not active
        let shield_active = shield_query.single()
            .map(|shield| shield.is_active && shield.length > 0.0)
            .unwrap_or(false);
        
        if !shield_active {
            energy.recharge(delta);
        }
        
        // Handle dash input (Space key)
        if input.just_pressed(KeyCode::Space) && dash.can_dash(&energy) {
            // Get current WASD direction for dash
            let mut dash_direction = Vec3::ZERO;
            
            if input.pressed(KeyCode::KeyW) || input.pressed(KeyCode::ArrowUp) {
                dash_direction += Vec3::Y;
            }
            if input.pressed(KeyCode::KeyS) || input.pressed(KeyCode::ArrowDown) {
                dash_direction -= Vec3::Y;
            }
            if input.pressed(KeyCode::KeyA) || input.pressed(KeyCode::ArrowLeft) {
                dash_direction -= Vec3::X;
            }
            if input.pressed(KeyCode::KeyD) || input.pressed(KeyCode::ArrowRight) {
                dash_direction += Vec3::X;
            }
            
            // Only dash if there's a direction
            if dash_direction != Vec3::ZERO {
                energy.consume(dash.energy_cost);
                dash.start_dash(dash_direction, player_transform.translation);
                info!("Player dash started! Energy: {:.1}/{:.1}", energy.current, energy.max);
            }
        }
        
        // Handle movement - either dash or regular movement
        if dash.is_dashing {
            // Dash movement
            let direction = (dash.dash_target - player_transform.translation).normalize_or_zero();
            let move_distance = dash.dash_speed * delta;
            player_transform.translation += direction * move_distance;
            
            // Check if dash is complete
            if dash.update_dash(player_transform.translation, delta) {
                info!("Player dash completed");
            }
        } else {
            // Regular movement with WASD
            let mut movement = Vec3::ZERO;
            let base_speed = 300.0; // Base movement speed
            
            if input.pressed(KeyCode::KeyW) || input.pressed(KeyCode::ArrowUp) {
                movement += Vec3::Y;
            }
            if input.pressed(KeyCode::KeyS) || input.pressed(KeyCode::ArrowDown) {
                movement -= Vec3::Y;
            }
            if input.pressed(KeyCode::KeyA) || input.pressed(KeyCode::ArrowLeft) {
                movement -= Vec3::X;
            }
            if input.pressed(KeyCode::KeyD) || input.pressed(KeyCode::ArrowRight) {
                movement += Vec3::X;
            }
            
            if movement != Vec3::ZERO {
                movement = movement.normalize() * base_speed * delta;
                player_transform.translation += movement;
            }
        }
    }
}

/// System to manage player invulnerability during dash
pub fn manage_player_invulnerability(
    mut player_query: Query<(&Transform, &PlayerDash, &mut Invulnerability), With<Player>>,
) {
    for (transform, dash, mut invulnerability) in &mut player_query {
        if dash.should_be_invulnerable(transform.translation) {
            invulnerability.activate();
        } else {
            invulnerability.deactivate();
        }
    }
}

/// System to handle player face mouse direction (only when not dashing)
pub fn player_face_mouse(
    mut player_query: Query<(&mut Transform, &PlayerDash), (With<Player>, Without<DirectionIndicator>)>,
    mut indicator_query: Query<&mut Transform, (With<DirectionIndicator>, Without<Player>)>,
    window_query: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    if let Ok(window) = window_query.single() {
        if let Ok((camera, camera_transform)) = camera_query.single() {
            if let Some(cursor_position) = window.cursor_position() {
                // Convert screen coordinates to world coordinates
                if let Ok(world_position) = camera.viewport_to_world_2d(camera_transform, cursor_position) {
                    for (mut player_transform, dash) in &mut player_query {
                        // Only rotate to face mouse when not dashing
                        if !dash.is_dashing {
                            let direction = (world_position - player_transform.translation.truncate()).normalize();
                            let angle = direction.y.atan2(direction.x);
                            player_transform.rotation = Quat::from_rotation_z(angle);
                            
                            // Update direction indicator position (if exists)
                            if let Ok(mut indicator_transform) = indicator_query.single_mut() {
                                // Position the indicator at a constant distance from player edge
                                let player_radius = 25.0;
                                let gap_from_edge = 7.0;
                                let distance_from_center = player_radius + gap_from_edge;
                                
                                // Since the player rotates, but we want the indicator to point in world direction,
                                // we need to counter-rotate the local position by the player's rotation
                                let inverse_rotation = player_transform.rotation.inverse();
                                let world_direction = Vec3::new(direction.x, direction.y, 0.0);
                                let local_direction = inverse_rotation * world_direction;
                                
                                // Set local position relative to parent (player)
                                indicator_transform.translation = Vec3::new(
                                    local_direction.x * distance_from_center,
                                    local_direction.y * distance_from_center,
                                    0.1 // Slightly above parent
                                );
                            }
                        }
                    }
                }
            }
        }
    }
}
