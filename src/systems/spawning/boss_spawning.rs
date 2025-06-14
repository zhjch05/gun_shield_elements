use bevy::prelude::*;
use crate::components::{Boss, MineBossBundle, MineSpikeBundle, DebugEntity};
use std::f32::consts::PI;

/// System to spawn Mine boss in debug mode
pub fn spawn_mine_boss(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Create mesh and material for the mine body (circle)
    let body_mesh = meshes.add(Circle::new(30.0)); // Main body radius
    let body_material = materials.add(Color::srgb(0.8, 0.4, 0.2)); // Orange-brown color
    
    // Create mesh and material for the spikes (squares)
    let spike_size = 12.0;
    let spike_mesh = meshes.add(Rectangle::new(spike_size, spike_size));
    let spike_material = materials.add(Color::srgb(0.6, 0.3, 0.1)); // Darker orange-brown
    
    // Spawn position for the mine boss
    let boss_position = Vec3::new(200.0, 200.0, 1.0);
    
    // Spawn mine boss body
    let boss_entity = commands.spawn((
        MineBossBundle::new(
            10.0,                      // max health
            200.0,
            boss_position,
            body_mesh,
            body_material,
        ),
        DebugEntity, // Mark as debug entity for cleanup
    )).id();
    
    // Spawn 8 spikes around the mine body
    let spike_distance = 42.0; // Distance from center to spike center
    commands.entity(boss_entity).with_children(|parent| {
        for i in 0..8 {
            let angle = (i as f32) * (PI / 4.0); // 8 evenly spaced angles
            let x = angle.cos() * spike_distance;
            let y = angle.sin() * spike_distance;
            
            parent.spawn((
                MineSpikeBundle::new(
                    Vec3::new(x, y, 0.1), // Local position relative to parent
                    spike_mesh.clone(),
                    spike_material.clone(),
                ),
                DebugEntity, // Mark as debug entity for cleanup
            ));
        }
    });
}

/// System to clean up boss entities
pub fn cleanup_boss_entities(mut commands: Commands, boss_query: Query<Entity, With<Boss>>) {
    for entity in &boss_query {
        commands.entity(entity).despawn(); // Despawn boss and all children (spikes)
    }
} 