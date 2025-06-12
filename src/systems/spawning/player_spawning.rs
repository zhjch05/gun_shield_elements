use bevy::prelude::*;
use crate::components::{Player, PlayerBundle};

/// System to spawn the player in debug mode
pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Create mesh and material
    let mesh = meshes.add(Circle::new(20.0));
    let material = materials.add(Color::WHITE);
    
    // Spawn player with proper z-index to ensure visibility
    commands.spawn(PlayerBundle::new(
        100.0,                                    // max health
        300.0,                                    // speed
        Vec3::new(0.0, 0.0, 10.0),               // position with higher z-index
        mesh,
        material,
    ));
    
    info!("Player spawned in debug mode");
}

/// System to clean up player entities
pub fn cleanup_player(mut commands: Commands, player_query: Query<Entity, With<Player>>) {
    for entity in &player_query {
        commands.entity(entity).despawn();
    }
    info!("Player entities cleaned up");
}
