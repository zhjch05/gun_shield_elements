use bevy::prelude::*;
use crate::components::{Player, Weapon, Projectile, ProjectileBundle, Boss, Health, Speed, Collider};

/// System to handle automatic weapon firing
pub fn weapon_firing_system(
    mut commands: Commands,
    mut weapon_query: Query<&mut Weapon>,
    player_query: Query<&Transform, With<Player>>,
    indicator_query: Query<&GlobalTransform, (With<crate::components::DirectionIndicator>, Without<Player>)>,
    window_query: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if let Ok(mut weapon) = weapon_query.single_mut() {
        // Update the weapon's internal timer
        weapon.update(time.delta());
        
        // In debug mode, weapon automatically fires constantly
        weapon.is_firing = true;
        
        if weapon.is_firing && weapon.can_fire() {
            // Get direction indicator position and mouse position
            if let (Ok(player_transform), Ok(indicator_transform), Ok(window)) = 
                (player_query.single(), indicator_query.single(), window_query.single()) {
                
                if let Ok((camera, camera_transform)) = camera_query.single() {
                    if let Some(cursor_position) = window.cursor_position() {
                        // Convert screen coordinates to world coordinates
                        if let Ok(mouse_world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_position) {
                            // Fire from direction indicator position
                            let spawn_position = indicator_transform.translation();
                            
                            // Calculate direction towards mouse
                            let direction = (mouse_world_pos.extend(0.0) - spawn_position).normalize();
                            
                            // Create projectile mesh and material
                            let projectile_mesh = meshes.add(Circle::new(3.0));
                            let projectile_material = materials.add(ColorMaterial::from(Color::WHITE));
                            
                            // Spawn projectile
                            commands.spawn(ProjectileBundle::new(
                                weapon.damage,
                                direction,
                                weapon.projectile_speed,
                                3.0, // 3 second lifetime
                                spawn_position,
                                projectile_mesh,
                                projectile_material,
                            ));
                            
                            weapon.fire();
                        }
                    }
                }
            }
        }
    }
}

/// System to move projectiles
pub fn projectile_movement_system(
    mut projectile_query: Query<(&mut Transform, &Projectile, &Speed)>,
    time: Res<Time>,
) {
    for (mut transform, projectile, speed) in projectile_query.iter_mut() {
        let movement = projectile.direction * speed.value * time.delta_secs();
        transform.translation += movement;
    }
}

/// System to update projectile lifetimes and despawn expired ones
pub fn projectile_lifetime_system(
    mut commands: Commands,
    mut projectile_query: Query<(Entity, &mut Projectile)>,
    time: Res<Time>,
) {
    for (entity, mut projectile) in projectile_query.iter_mut() {
        if projectile.update(time.delta()) {
            // Projectile lifetime expired
            commands.entity(entity).despawn();
        }
    }
}

/// System to handle projectile collision with bosses
pub fn projectile_boss_collision_system(
    mut commands: Commands,
    projectile_query: Query<(Entity, &Transform, &Projectile, &Collider), Without<Boss>>,
    mut boss_query: Query<(&Transform, &mut Health, &Collider), With<Boss>>,
) {
    for (projectile_entity, projectile_transform, projectile, projectile_collider) in projectile_query.iter() {
        for (boss_transform, mut boss_health, boss_collider) in boss_query.iter_mut() {
            let distance = projectile_transform.translation.distance(boss_transform.translation);
            let collision_radius = projectile_collider.radius + boss_collider.radius;
            
            if distance < collision_radius {
                // Deal damage to boss
                boss_health.take_damage(projectile.damage);
                
                info!("Projectile hit boss for {} damage! Boss health: {:.1}/{:.1}", 
                    projectile.damage, boss_health.current, boss_health.max);
                
                // Despawn the projectile
                commands.entity(projectile_entity).despawn();
                
                if !boss_health.is_alive() {
                    info!("Boss has been defeated!");
                    // TODO: Handle boss death (drop loot, spawn effects, etc.)
                }
            }
        }
    }
}

/// System to clean up projectiles
pub fn cleanup_projectiles(
    mut commands: Commands,
    projectile_query: Query<Entity, With<Projectile>>,
) {
    for entity in projectile_query.iter() {
        commands.entity(entity).despawn();
    }
} 