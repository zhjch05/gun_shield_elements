use bevy::prelude::*;
use crate::components::attributes::{Health, Speed};

/// Marker component for the player entity
#[derive(Component, Debug)]
pub struct Player;

/// Marker component for the direction indicator that shows where the player is facing
#[derive(Component, Debug)]
pub struct DirectionIndicator;

/// Bundle for player with all necessary components
#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub health: Health,
    pub speed: Speed,
    pub transform: Transform,
    pub mesh: Mesh2d,
    pub material: MeshMaterial2d<ColorMaterial>,
}

impl PlayerBundle {
    pub fn new(
        max_health: f32,
        speed: f32,
        position: Vec3,
        mesh: Handle<Mesh>,
        material: Handle<ColorMaterial>,
    ) -> Self {
        Self {
            player: Player,
            health: Health::new(max_health),
            speed: Speed::new(speed),
            transform: Transform::from_translation(position),
            mesh: Mesh2d(mesh),
            material: MeshMaterial2d(material),
        }
    }
}

/// Bundle for direction indicator
#[derive(Bundle)]
pub struct DirectionIndicatorBundle {
    pub indicator: DirectionIndicator,
    pub transform: Transform,
    pub mesh: Mesh2d,
    pub material: MeshMaterial2d<ColorMaterial>,
}

impl DirectionIndicatorBundle {
    pub fn new(
        mesh: Handle<Mesh>,
        material: Handle<ColorMaterial>,
    ) -> Self {
        Self {
            indicator: DirectionIndicator,
            transform: Transform::from_translation(Vec3::new(32.0, 0.0, 0.1)), // Local position relative to parent player (25.0 radius + 7.0 gap)
            mesh: Mesh2d(mesh),
            material: MeshMaterial2d(material),
        }
    }
}
