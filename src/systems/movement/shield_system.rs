use bevy::prelude::*;
use std::f32::consts::PI;
use crate::components::{Player, Shield, DirectionIndicator};
use crate::components::attributes::Energy;

/// System to handle shield input (right click)
pub fn handle_shield_input(
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut shield_query: Query<&mut Shield>,
    mut energy_query: Query<&mut Energy, With<Player>>,
    mut indicator_query: Query<&mut Visibility, With<DirectionIndicator>>,
) {
    if let Ok(mut shield) = shield_query.single_mut() {
        if let Ok(mut energy) = energy_query.single_mut() {
            if mouse_input.just_pressed(MouseButton::Right) {
                // Check if we have enough energy to activate the shield
                if shield.can_activate(&energy) {
                    // Consume the activation energy cost
                    energy.consume(shield.activation_energy_cost);
                    shield.activate(0.5); // Full shield length
                    
                    // Hide direction indicator when shield is active
                    if let Ok(mut visibility) = indicator_query.single_mut() {
                        *visibility = Visibility::Hidden;
                    }
                }
            } else if mouse_input.just_released(MouseButton::Right) {
                shield.deactivate();
                
                // Show direction indicator when shield is deactivated
                if let Ok(mut visibility) = indicator_query.single_mut() {
                    *visibility = Visibility::Visible;
                }
            }
        }
    }
}

/// System to animate shield growth/shrinkage and handle energy consumption
pub fn animate_shield(
    time: Res<Time<Virtual>>,
    mut shield_query: Query<&mut Shield>,
    mut energy_query: Query<&mut Energy, With<Player>>,
    mut indicator_query: Query<&mut Visibility, With<DirectionIndicator>>,
) {
    for mut shield in &mut shield_query {
        if let Ok(mut energy) = energy_query.single_mut() {
            // If shield is active, consume energy
            if shield.is_active && shield.length > 0.0 {
                let energy_to_consume = shield.energy_drain_rate * time.delta_secs();
                
                // If we can't consume energy, deactivate the shield
                if !energy.consume(energy_to_consume) {
                    shield.deactivate();
                }
            }
        }
        
        if shield.is_active || shield.length > 0.0 {
            let delta = time.delta_secs() * shield.animation_speed;
            
            if shield.length < shield.target_length {
                // Growing - ensure direction indicator is hidden
                shield.length = (shield.length + delta).min(shield.target_length);
                if let Ok(mut visibility) = indicator_query.single_mut() {
                    *visibility = Visibility::Hidden;
                }
            } else if shield.length > shield.target_length {
                // Shrinking
                shield.length = (shield.length - delta).max(shield.target_length);
                
                // Deactivate when fully shrunk and show direction indicator
                if shield.length <= 0.0 {
                    shield.is_active = false;
                    if let Ok(mut visibility) = indicator_query.single_mut() {
                        *visibility = Visibility::Visible;
                    }
                }
            }
        }
    }
}

/// System to update shield mesh based on current length
pub fn update_shield_mesh(
    mut meshes: ResMut<Assets<Mesh>>,
    shield_query: Query<(&Shield, &Mesh2d), Changed<Shield>>,
    player_query: Query<&Transform, (With<Player>, Without<DirectionIndicator>)>,
    indicator_query: Query<&Transform, (With<DirectionIndicator>, Without<Player>)>,
) {
    if let Ok(_player_transform) = player_query.single() {
        if let Ok(indicator_transform) = indicator_query.single() {
            for (shield, mesh_handle) in &shield_query {
                if shield.length > 0.0 {
                    // Calculate the angle from player center to direction indicator in local space
                    let indicator_local_pos = indicator_transform.translation.truncate();
                    let center_angle = indicator_local_pos.y.atan2(indicator_local_pos.x);
                    
                    // Generate arc mesh centered on the direction indicator
                    let mesh = create_shield_arc_mesh(shield.length, center_angle);
                    meshes.insert(&mesh_handle.0, mesh);
                } else {
                    // Clear the mesh when shield length is 0 or less
                    let empty_mesh = create_empty_mesh();
                    meshes.insert(&mesh_handle.0, empty_mesh);
                }
            }
        }
    }
}

/// Create a mesh for the shield arc centered on the direction indicator
fn create_shield_arc_mesh(length: f32, center_angle: f32) -> Mesh {
    let player_radius = 25.0;
    let shield_thickness = 4.0; // Reduced back to 4.0
    let gap_from_edge = 7.0; // Same as direction indicator positioning
    let shield_radius = player_radius + gap_from_edge; // 32.0 - same as direction indicator distance
    
    // Calculate arc parameters - arc grows equally in both directions from center
    let total_arc_length = length * 2.0 * PI; // Full circle when length = 1.0
    let half_arc_length = total_arc_length / 2.0;
    let start_angle = center_angle - half_arc_length;
    
    // Number of segments for smooth arc
    let segments = ((total_arc_length / PI * 16.0) as usize).max(2);
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();
    
    // Generate vertices for the arc
    for i in 0..=segments {
        let t = i as f32 / segments as f32;
        let angle = start_angle + t * total_arc_length;
        
        let cos_a = angle.cos();
        let sin_a = angle.sin();
        
        // Inner arc point
        let inner_x = cos_a * (shield_radius - shield_thickness / 2.0);
        let inner_y = sin_a * (shield_radius - shield_thickness / 2.0);
        vertices.push([inner_x, inner_y, 0.0]);
        normals.push([0.0, 0.0, 1.0]);
        uvs.push([0.0, t]);
        
        // Outer arc point
        let outer_x = cos_a * (shield_radius + shield_thickness / 2.0);
        let outer_y = sin_a * (shield_radius + shield_thickness / 2.0);
        vertices.push([outer_x, outer_y, 0.0]);
        normals.push([0.0, 0.0, 1.0]);
        uvs.push([1.0, t]);
        
        // Create triangles (except for last iteration)
        if i < segments {
            let base = i * 2;
            // First triangle
            indices.push(base as u32);
            indices.push((base + 1) as u32);
            indices.push((base + 2) as u32);
            
            // Second triangle
            indices.push((base + 1) as u32);
            indices.push((base + 3) as u32);
            indices.push((base + 2) as u32);
        }
    }
    
    let mut mesh = Mesh::new(
        bevy::render::render_resource::PrimitiveTopology::TriangleList,
        bevy::render::render_asset::RenderAssetUsages::MAIN_WORLD | bevy::render::render_asset::RenderAssetUsages::RENDER_WORLD,
    );
    
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(bevy::render::mesh::Indices::U32(indices));
    
    mesh
}

/// Create an empty mesh to clear the shield when not active
fn create_empty_mesh() -> Mesh {
    let mut mesh = Mesh::new(
        bevy::render::render_resource::PrimitiveTopology::TriangleList,
        bevy::render::render_asset::RenderAssetUsages::MAIN_WORLD | bevy::render::render_asset::RenderAssetUsages::RENDER_WORLD,
    );
    
    // Empty vertices, indices, normals, and UVs
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, Vec::<[f32; 3]>::new());
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, Vec::<[f32; 3]>::new());
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, Vec::<[f32; 2]>::new());
    mesh.insert_indices(bevy::render::mesh::Indices::U32(Vec::<u32>::new()));
    
    mesh
} 