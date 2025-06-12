use bevy::prelude::*;
use crate::components::{Player, PlayerBundle, DebugEntity, DirectionIndicatorBundle};

/// System to spawn the player in debug mode
pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Create mesh and material with better visibility
    let player_mesh = meshes.add(Circle::new(25.0)); // Slightly larger for better visibility
    let player_material = materials.add(Color::WHITE); // White color for player
    
    // Create center marker
    let marker_mesh = meshes.add(Circle::new(8.0)); // Small circle for center marker
    let marker_material = materials.add(Color::srgb(1.0, 0.2, 0.2)); // Bright red
    
    // Create direction indicator
    let indicator_mesh = meshes.add(Circle::new(4.0)); // Tiny white circle
    let indicator_material = materials.add(Color::WHITE); // White color for direction indicator
    
    // Spawn center marker at origin
    commands.spawn((
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.5)), // Slightly below player
        Mesh2d(marker_mesh),
        MeshMaterial2d(marker_material),
        DebugEntity, // Mark as debug entity for cleanup
    ));
    
    // Spawn player with proper positioning and debug marker
    let player_entity = commands.spawn((
        PlayerBundle::new(
            100.0,                                    // max health
            300.0,                                    // speed
            Vec3::new(0.0, 0.0, 1.0),                // position with proper z-index
            player_mesh,
            player_material,
        ),
        DebugEntity, // Mark as debug entity for cleanup
    )).id();
    
    // Spawn direction indicator as a child of the player
    commands.entity(player_entity).with_children(|parent| {
        parent.spawn((
            DirectionIndicatorBundle::new(
                indicator_mesh,
                indicator_material,
            ),
            DebugEntity, // Mark as debug entity for cleanup
        ));
    });
    
    info!("Player spawned in debug mode at position (0, 0) with center marker and direction indicator as child");
}

/// System to clean up player entities
pub fn cleanup_player(mut commands: Commands, player_query: Query<Entity, With<Player>>) {
    for entity in &player_query {
        commands.entity(entity).despawn(); // Automatically despawns children
    }
    info!("Player entities cleaned up");
}

/// System to clean up debug entities
pub fn cleanup_debug_entities(mut commands: Commands, debug_query: Query<Entity, With<DebugEntity>>) {
    for entity in &debug_query {
        commands.entity(entity).despawn();
    }
    info!("Debug entities cleaned up");
}
