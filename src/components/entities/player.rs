use bevy::prelude::*;
use crate::components::attributes::{Health, Speed, Collider, Energy, Invulnerability};

/// Marker component for the player entity
#[derive(Component, Debug)]
pub struct Player;

/// Marker component for the direction indicator that shows where the player is facing
#[derive(Component, Debug)]
pub struct DirectionIndicator;

/// Component for player dash abilities
#[derive(Component, Debug)]
pub struct PlayerDash {
    pub is_dashing: bool,
    pub dash_target: Vec3,
    pub dash_start_position: Vec3,
    pub dash_speed: f32,
    pub dash_distance: f32,
    pub energy_cost: f32, // Energy required per dash
    pub invulnerability_percentage: f32, // Percentage of dash distance that provides invulnerability (0.0 to 1.0)
}

impl Default for PlayerDash {
    fn default() -> Self {
        Self {
            is_dashing: false,
            dash_target: Vec3::ZERO,
            dash_start_position: Vec3::ZERO,
            dash_speed: 800.0, // Faster than boss dash
            dash_distance: 400.0, // Reasonable dash distance for player
            energy_cost: 30.0, // Energy cost per dash
            invulnerability_percentage: 0.3, // First 30% of dash provides invulnerability
        }
    }
}

impl PlayerDash {
    pub fn new(dash_speed: f32, dash_distance: f32, energy_cost: f32, invulnerability_percentage: f32) -> Self {
        Self {
            is_dashing: false,
            dash_target: Vec3::ZERO,
            dash_start_position: Vec3::ZERO,
            dash_speed,
            dash_distance,
            energy_cost,
            invulnerability_percentage: invulnerability_percentage.clamp(0.0, 1.0),
        }
    }

    pub fn can_dash(&self, energy: &Energy) -> bool {
        !self.is_dashing && energy.can_consume(self.energy_cost)
    }

    pub fn start_dash(&mut self, direction: Vec3, start_position: Vec3) {
        self.is_dashing = true;
        self.dash_target = start_position + direction.normalize_or_zero() * self.dash_distance;
        self.dash_start_position = start_position;
    }

    pub fn update_dash(&mut self, current_position: Vec3) -> bool {
        if self.is_dashing {
            // Check if we've traveled the full dash distance
            let distance_traveled = self.dash_start_position.distance(current_position);
            if distance_traveled >= self.dash_distance {
                self.is_dashing = false;
                return true; // Dash completed
            }
        }
        false
    }

    /// Check if the player should be invulnerable based on dash progress
    pub fn should_be_invulnerable(&self, current_position: Vec3) -> bool {
        if !self.is_dashing {
            return false;
        }
        
        let distance_traveled = self.dash_start_position.distance(current_position);
        let invulnerable_distance = self.dash_distance * self.invulnerability_percentage;
        
        distance_traveled <= invulnerable_distance
    }
}

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
    /// Energy consumed per second while shield is active
    pub energy_drain_rate: f32,
    /// Minimum energy required to activate shield
    pub activation_energy_cost: f32,
}

impl Shield {
    pub fn new() -> Self {
        Self {
            length: 0.0,
            target_length: 0.0,
            animation_speed: 3.0, // Units per second
            is_active: false,
            energy_drain_rate: 20.0, // Energy per second while active
            activation_energy_cost: 10.0, // Minimum energy required to activate
        }
    }

    pub fn activate(&mut self, target_length: f32) {
        self.is_active = true;
        self.target_length = target_length;
    }

    pub fn deactivate(&mut self) {
        self.target_length = 0.0;
    }

    pub fn can_activate(&self, energy: &Energy) -> bool {
        energy.can_consume(self.activation_energy_cost)
    }
}

/// Bundle for player with all necessary components
#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub health: Health,
    pub speed: Speed,
    pub energy: Energy,
    pub collider: Collider,
    pub dash: PlayerDash,
    pub invulnerability: Invulnerability,
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
            energy: Energy::new(100.0, 25.0), // 100 max energy, 25 per second recharge
            collider: Collider::new(25.0), // Player radius
            dash: PlayerDash::default(),
            invulnerability: Invulnerability::new(),
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
