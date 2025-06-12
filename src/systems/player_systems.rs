use bevy::prelude::*;
use crate::components::{Player, PlayerStatsBundle};

/// System to spawn the player in debug mode
pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Spawn player as a white circle
    commands.spawn((
        // Visual representation
        Mesh2d(meshes.add(Circle::new(20.0))),
        MeshMaterial2d(materials.add(Color::WHITE)),

        // Transform (position, rotation, scale)
        Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),

        // Player components
        Player,
        PlayerStatsBundle::new(100.0, 300.0), // 100 health, 300 speed
    ));
}

/// System to handle player movement with WASD keys
pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<(&mut Transform, &crate::components::Speed), With<Player>>,
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

/// System to clean up player entities
pub fn cleanup_player(mut commands: Commands, player_query: Query<Entity, With<Player>>) {
    for entity in &player_query {
        commands.entity(entity).despawn();
    }
}
