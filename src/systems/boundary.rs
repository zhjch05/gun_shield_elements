use bevy::prelude::*;
use crate::components::{BoundedMovement, BoundaryVisual, EdgeWarning, EdgeType, Player, PlayerDash};
use crate::constants::{GameBoundaries, AppColors};

/// System to enforce boundary constraints on all bounded entities
pub fn enforce_boundaries(
    mut bounded_query: Query<(&mut Transform, Option<&mut PlayerDash>), With<BoundedMovement>>,
) {
    for (mut transform, dash_opt) in bounded_query.iter_mut() {
        let original_pos = transform.translation;
        let clamped_pos = GameBoundaries::clamp_position(original_pos);
        
        // If position was clamped and this is a player with dash, reset the dash to prevent getting stuck
        if original_pos != clamped_pos {
            if let Some(mut dash) = dash_opt {
                if dash.is_dashing {
                    dash.reset_dash(); // Properly reset all dash state when hitting boundary
                }
            }
            transform.translation = clamped_pos;
        }
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

/// System to spawn edge warning UI elements
pub fn spawn_edge_warnings(
    mut commands: Commands,
    existing_warnings: Query<Entity, With<EdgeWarning>>,
) {
    // Only spawn if warnings don't exist yet
    if !existing_warnings.is_empty() {
        return;
    }

    let warning_thickness = 30.0; // How thick the warning bloom should be
    
    // Top edge warning
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(0.0),
            left: Val::Px(0.0),
            width: Val::Percent(100.0),
            height: Val::Px(warning_thickness),
            ..default()
        },
        BackgroundColor(Color::srgba(1.0, 0.0, 0.0, 0.0)), // Start transparent
        ZIndex(50), // Above game but below UI
        EdgeWarning { edge: EdgeType::Top },
    ));
    
    // Bottom edge warning
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(0.0),
            left: Val::Px(0.0),
            width: Val::Percent(100.0),
            height: Val::Px(warning_thickness),
            ..default()
        },
        BackgroundColor(Color::srgba(1.0, 0.0, 0.0, 0.0)), // Start transparent
        ZIndex(50), // Above game but below UI
        EdgeWarning { edge: EdgeType::Bottom },
    ));
    
    // Left edge warning
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(0.0),
            left: Val::Px(0.0),
            width: Val::Px(warning_thickness),
            height: Val::Percent(100.0),
            ..default()
        },
        BackgroundColor(Color::srgba(1.0, 0.0, 0.0, 0.0)), // Start transparent
        ZIndex(50), // Above game but below UI
        EdgeWarning { edge: EdgeType::Left },
    ));
    
    // Right edge warning
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(0.0),
            right: Val::Px(0.0),
            width: Val::Px(warning_thickness),
            height: Val::Percent(100.0),
            ..default()
        },
        BackgroundColor(Color::srgba(1.0, 0.0, 0.0, 0.0)), // Start transparent
        ZIndex(50), // Above game but below UI
        EdgeWarning { edge: EdgeType::Right },
    ));
}

/// System to update edge warning intensity based on player proximity to boundaries
pub fn update_edge_warnings(
    player_query: Query<&Transform, With<Player>>,
    mut warning_query: Query<(&EdgeWarning, &mut BackgroundColor)>,
    windows: Query<&Window>,
) {
    if let Ok(player_transform) = player_query.single() {
        let player_pos = player_transform.translation;
        
        // Get window size for calculations (fallback if window query fails)
        let (_viewport_width, _viewport_height) = if let Ok(window) = windows.single() {
            (window.width(), window.height())
        } else {
            (1920.0, 1080.0) // Default fallback
        };
        
        // Warning distance - how far from boundary to start showing warning
        let warning_distance = 400.0; // Increased from 200 to show warning earlier
        
        for (warning, mut background_color) in warning_query.iter_mut() {
            let distance_to_edge = match warning.edge {
                EdgeType::Top => GameBoundaries::max_y() - player_pos.y,
                EdgeType::Bottom => player_pos.y - GameBoundaries::min_y(),
                EdgeType::Left => player_pos.x - GameBoundaries::min_x(),
                EdgeType::Right => GameBoundaries::max_x() - player_pos.x,
            };
            
            // Calculate warning intensity (0.0 = no warning, 1.0 = maximum warning)
            let warning_intensity = if distance_to_edge <= warning_distance {
                1.0 - (distance_to_edge / warning_distance).max(0.0)
            } else {
                0.0
            };
            
            // Apply intensity to alpha channel with a subtle red bloom
            let alpha = (warning_intensity * 0.3).min(0.3); // Cap at 30% opacity for subtlety
            *background_color = BackgroundColor(Color::srgba(1.0, 0.0, 0.0, alpha));
        }
    }
}

/// System to clean up boundary visuals and edge warnings
pub fn cleanup_boundary_visuals(
    mut commands: Commands,
    boundary_query: Query<Entity, With<BoundaryVisual>>,
) {
    for entity in boundary_query.iter() {
        commands.entity(entity).despawn();
    }
}

/// System to clean up edge warnings
pub fn cleanup_edge_warnings(
    mut commands: Commands,
    warning_query: Query<Entity, With<EdgeWarning>>,
) {
    for entity in warning_query.iter() {
        commands.entity(entity).despawn();
    }
} 