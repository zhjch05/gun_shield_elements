use bevy::prelude::*;
use crate::components::{Boss, MineBoss, BossSkills, RotationAnimation, Player, Health, CollisionDamage, Collider};

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
                    // Calculate direction to player
                    let direction = (player_transform.translation - boss_transform.translation).normalize_or_zero();
                    let dash_target = boss_transform.translation + direction * skills.dash_distance;
                    
                    skills.start_dash(dash_target, boss_transform.translation);
                    info!("Mine boss starting dash towards player at distance: {:.1}, dash distance: {:.1}", distance_to_player, skills.dash_distance);
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
                
                // Check if dash is complete (using current position)
                if skills.update_dash(transform.translation) {
                    info!("Mine boss dash completed");
                    rotation.stop();
                }
            } else {
                // Constant slow movement toward player when not dashing
                let direction = (player_transform.translation - transform.translation).normalize_or_zero();
                let move_distance = skills.constant_movement_speed * delta;
                
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
    mut boss_query: Query<(&Transform, &mut BossSkills, &Collider), (With<MineBoss>, Without<Player>)>,
    mut player_query: Query<(&Transform, &mut Health, &Collider), (With<Player>, Without<MineBoss>)>,
) {
    if let Ok((player_transform, mut player_health, player_collider)) = player_query.single_mut() {
        for (boss_transform, mut skills, boss_collider) in boss_query.iter_mut() {
            if skills.can_hit_player() {
                let distance = boss_transform.translation.distance(player_transform.translation);
                // Accurate circle-to-circle collision: sum of both radii
                let collision_radius = player_collider.radius + boss_collider.radius;
                
                if distance < collision_radius {
                    // Apply damage to player (only once per dash)
                    player_health.take_damage(skills.dash_damage);
                    skills.mark_player_hit(); // Mark that we've hit the player this dash
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

/// System to handle collision damage between boss and player during constant movement
pub fn boss_collision_damage(
    mut boss_query: Query<(&Transform, &BossSkills, &mut CollisionDamage, &Collider), (With<MineBoss>, Without<Player>)>,
    mut player_query: Query<(&Transform, &mut Health, &Collider), (With<Player>, Without<MineBoss>)>,
    time: Res<Time>,
) {
    if let Ok((player_transform, mut player_health, player_collider)) = player_query.single_mut() {
        for (boss_transform, skills, mut collision_damage, boss_collider) in boss_query.iter_mut() {
            // Only apply collision damage when NOT dashing (constant movement only)
            if !skills.is_dashing {
                let distance = boss_transform.translation.distance(player_transform.translation);
                // Accurate circle-to-circle collision: sum of both radii
                let collision_radius = player_collider.radius + boss_collider.radius;
                
                if distance < collision_radius {
                    let current_time = time.elapsed_secs();
                    if collision_damage.can_damage(current_time) {
                        let damage = collision_damage.apply_damage(current_time);
                        player_health.take_damage(damage);
                        info!("Boss collision damage: {} damage! Player health: {:.1}/{:.1}", 
                            damage, player_health.current, player_health.max);
                        
                        if !player_health.is_alive() {
                            info!("Player has died from collision damage!");
                        }
                    }
                }
            }
        }
    }
} 