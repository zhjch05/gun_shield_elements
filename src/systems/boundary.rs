use bevy::prelude::*;
use crate::components::{BoundedMovement, BoundaryVisual};
use crate::constants::{GameBoundaries, AppColors};

/// System to enforce boundary constraints on all bounded entities
pub fn enforce_boundaries(
    mut bounded_query: Query<&mut Transform, With<BoundedMovement>>,
) {
    for mut transform in bounded_query.iter_mut() {
        transform.translation = GameBoundaries::clamp_position(transform.translation);
    }
}

/// System to spawn visual boundary elements
pub fn spawn_boundary_visuals(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    existing_boundaries: Query<Entity, With<BoundaryVisual>>,
) {
    // Only spawn if boundaries don't exist yet
    if !existing_boundaries.is_empty() {
        return;
    }

    // Create boundary visual size constants
    let boundary_thickness = 100.0; // How thick the boundary zone should be
    let world_width = GameBoundaries::width();
    let world_height = GameBoundaries::height();
    
    // Black material for out-of-bounds areas
    let black_material = materials.add(Color::BLACK);
    
    // Gray material for the game area background
    let gray_material = materials.add(AppColors::BACKGROUND);
    
    // Create the game area background first (behind everything)
    let game_area_mesh = meshes.add(Rectangle::new(world_width, world_height));
    commands.spawn((
        Transform::from_translation(Vec3::new(0.0, 0.0, -2.0)), // Behind boundaries
        Mesh2d(game_area_mesh),
        MeshMaterial2d(gray_material),
        BoundaryVisual,
    ));
    
    // Create the four boundary rectangles around the game area
    
    // Top boundary
    let top_boundary_mesh = meshes.add(Rectangle::new(
        world_width + boundary_thickness * 2.0, 
        boundary_thickness
    ));
    commands.spawn((
        Transform::from_translation(Vec3::new(
            0.0, 
            GameBoundaries::max_y() + boundary_thickness / 2.0, 
            -1.0 // Behind game objects
        )),
        Mesh2d(top_boundary_mesh),
        MeshMaterial2d(black_material.clone()),
        BoundaryVisual,
    ));
    
    // Bottom boundary
    let bottom_boundary_mesh = meshes.add(Rectangle::new(
        world_width + boundary_thickness * 2.0, 
        boundary_thickness
    ));
    commands.spawn((
        Transform::from_translation(Vec3::new(
            0.0, 
            GameBoundaries::min_y() - boundary_thickness / 2.0, 
            -1.0 // Behind game objects
        )),
        Mesh2d(bottom_boundary_mesh),
        MeshMaterial2d(black_material.clone()),
        BoundaryVisual,
    ));
    
    // Left boundary
    let left_boundary_mesh = meshes.add(Rectangle::new(
        boundary_thickness, 
        world_height
    ));
    commands.spawn((
        Transform::from_translation(Vec3::new(
            GameBoundaries::min_x() - boundary_thickness / 2.0, 
            0.0, 
            -1.0 // Behind game objects
        )),
        Mesh2d(left_boundary_mesh),
        MeshMaterial2d(black_material.clone()),
        BoundaryVisual,
    ));
    
    // Right boundary
    let right_boundary_mesh = meshes.add(Rectangle::new(
        boundary_thickness, 
        world_height
    ));
    commands.spawn((
        Transform::from_translation(Vec3::new(
            GameBoundaries::max_x() + boundary_thickness / 2.0, 
            0.0, 
            -1.0 // Behind game objects
        )),
        Mesh2d(right_boundary_mesh),
        MeshMaterial2d(black_material),
        BoundaryVisual,
    ));
}

/// System to clean up boundary visuals
pub fn cleanup_boundary_visuals(
    mut commands: Commands,
    boundary_query: Query<Entity, With<BoundaryVisual>>,
) {
    for entity in boundary_query.iter() {
        commands.entity(entity).despawn();
    }
} 