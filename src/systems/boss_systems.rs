use bevy::prelude::*;
use crate::components::{Boss, MineBoss, BossSkills, RotationAnimation, Player, Health};

/// System to handle Mine boss AI and skill usage
pub fn mine_boss_ai(
    mut boss_query: Query<(&mut BossSkills, &Transform), (With<MineBoss>, Without<Player>)>,
    player_query: Query<&Transform, (With<Player>, Without<MineBoss>)>,
    time: Res<Time>,
) {
    if let Ok(player_transform) = player_query.single() {
        for (mut skills, boss_transform) in boss_query.iter_mut() {
            // Update skill cooldowns
            skills.dash_cooldown.tick(time.delta());
            
            // Check if boss should dash towards player
            if skills.can_dash() {
                let distance_to_player = boss_transform.translation.distance(player_transform.translation);
                
                // Dash if player is not too close (avoid dash when already very close)
                if distance_to_player > 100.0 {
                    skills.start_dash(player_transform.translation);
                    info!("Mine boss starting dash towards player at distance: {:.1}", distance_to_player);
                }
            }
        }
    }
}

/// System to handle boss dash movement and animation
pub fn boss_dash_movement(
    mut boss_query: Query<(&mut Transform, &mut BossSkills, &mut RotationAnimation), With<MineBoss>>,
    player_query: Query<&Transform, (With<Player>, Without<MineBoss>)>,
    time: Res<Time>,
) {
    if let Ok(player_transform) = player_query.single() {
        for (mut transform, mut skills, mut rotation) in boss_query.iter_mut() {
            let delta = time.delta_secs();
            
            if skills.is_dashing {
                // Start rotation animation during dash
                if !rotation.enabled {
                    rotation.start();
                }
                
                // Calculate direction and move towards target
                let direction = (skills.dash_target - transform.translation).normalize_or_zero();
                let move_distance = skills.dash_speed * delta;
                
                // Move towards target
                transform.translation += direction * move_distance;
                
                // Check if dash is complete
                if skills.update_dash(delta) {
                    info!("Mine boss dash completed");
                    rotation.stop();
                }
            } else {
                // Constant slow movement toward player when not dashing
                let direction = (player_transform.translation - transform.translation).normalize_or_zero();
                let constant_speed = 50.0; // Slow constant movement speed
                let move_distance = constant_speed * delta;
                
                // Move towards player
                transform.translation += direction * move_distance;
                
                // Stop rotation when not dashing
                rotation.stop();
            }
        }
    }
}

/// System to handle rotation animation
pub fn boss_rotation_animation(
    mut boss_query: Query<(&mut Transform, &RotationAnimation), With<Boss>>,
    time: Res<Time>,
) {
    for (mut transform, rotation) in boss_query.iter_mut() {
        if rotation.enabled {
            let rotation_delta = rotation.speed * time.delta_secs();
            transform.rotate_z(rotation_delta);
        }
    }
}

/// System to handle collision between boss and player
pub fn boss_player_collision(
    boss_query: Query<(&Transform, &BossSkills), (With<MineBoss>, Without<Player>)>,
    mut player_query: Query<(&Transform, &mut Health), (With<Player>, Without<MineBoss>)>,
) {
    if let Ok((player_transform, mut player_health)) = player_query.single_mut() {
        for (boss_transform, skills) in boss_query.iter() {
            if skills.is_dashing {
                let distance = boss_transform.translation.distance(player_transform.translation);
                let collision_radius = 35.0; // Boss radius (30) + small buffer
                
                if distance < collision_radius {
                    // Apply damage to player
                    player_health.take_damage(skills.dash_damage);
                    info!("Mine boss hit player for {} damage! Player health: {:.1}/{:.1}", 
                        skills.dash_damage, player_health.current, player_health.max);
                    
                    if !player_health.is_alive() {
                        info!("Player has died!");
                        // TODO: Handle player death (restart, game over screen, etc.)
                    }
                }
            }
        }
    }
} 