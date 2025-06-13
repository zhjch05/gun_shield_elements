use bevy::prelude::*;
use crate::components::{Player, PlayerBundle, DebugEntity, DirectionIndicatorBundle, ShieldBundle, WeaponBundle};

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
    let indicator_mesh = meshes.add(Circle::new(2.0)); // Diameter 4.0 to match shield thickness
    let indicator_material = materials.add(Color::WHITE); // White color for direction indicator
    
    // Create shield (starts with empty mesh, will be generated dynamically)
    let shield_mesh = meshes.add(Circle::new(0.0)); // Empty mesh initially
    let shield_material = materials.add(Color::WHITE); // White shield color
    
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
    
    // Spawn direction indicator, shield, and weapon as children of the player
    commands.entity(player_entity).with_children(|parent| {
        // Direction indicator
        parent.spawn((
            DirectionIndicatorBundle::new(
                indicator_mesh,
                indicator_material,
            ),
            DebugEntity, // Mark as debug entity for cleanup
        ));
        
        // Shield
        parent.spawn((
            ShieldBundle::new(
                shield_mesh,
                shield_material,
            ),
            DebugEntity, // Mark as debug entity for cleanup
        ));
        
        // Weapon - Default gun in debug mode
        parent.spawn((
            WeaponBundle::new(
                5.0, // 5 shots per second
                10.0, // 10 damage per shot
                400.0, // 400 units per second projectile speed
            ),
            DebugEntity, // Mark as debug entity for cleanup
        ));
    });
    
 }

/// System to clean up player entities
pub fn cleanup_player(mut commands: Commands, player_query: Query<Entity, With<Player>>) {
    for entity in &player_query {
        commands.entity(entity).despawn(); // Automatically despawns children
    }
}

/// System to clean up debug entities
pub fn cleanup_debug_entities(mut commands: Commands, debug_query: Query<Entity, With<DebugEntity>>) {
    for entity in &debug_query {
        commands.entity(entity).despawn();
    }
}
