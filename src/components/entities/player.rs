use bevy::prelude::*;
use crate::components::attributes::{Health, Speed};

/// Marker component for the player entity
#[derive(Component, Debug)]
pub struct Player;

/// Marker component for the direction indicator that shows where the player is facing
#[derive(Component, Debug)]
pub struct DirectionIndicator;

/// Component for the shield system
#[derive(Component, Debug)]
pub struct Shield {
    /// Length of the shield arc, from 0.0 (just indicator dot) to 1.0 (full circle)
    pub length: f32,
    /// Target length for animation
    pub target_length: f32,
    /// Animation speed for growing/shrinking
    pub animation_speed: f32,
    /// Whether the shield is currently active
    pub is_active: bool,
}

impl Shield {
    pub fn new() -> Self {
        Self {
            length: 0.0,
            target_length: 0.0,
            animation_speed: 3.0, // Units per second
            is_active: false,
        }
    }

    pub fn activate(&mut self, target_length: f32) {
        self.is_active = true;
        self.target_length = target_length;
    }

    pub fn deactivate(&mut self) {
        self.target_length = 0.0;
    }
}

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

/// Bundle for shield entity
#[derive(Bundle)]
pub struct ShieldBundle {
    pub shield: Shield,
    pub transform: Transform,
    pub mesh: Mesh2d,
    pub material: MeshMaterial2d<ColorMaterial>,
}

impl ShieldBundle {
    pub fn new(
        mesh: Handle<Mesh>,
        material: Handle<ColorMaterial>,
    ) -> Self {
        Self {
            shield: Shield::new(),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.05)), // Local position relative to parent player
            mesh: Mesh2d(mesh),
            material: MeshMaterial2d(material),
        }
    }
}
