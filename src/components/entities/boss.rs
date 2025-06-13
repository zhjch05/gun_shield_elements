use bevy::prelude::*;
use crate::components::attributes::{Health, Speed};

/// Marker component for all boss entities
#[derive(Component, Debug)]
pub struct Boss;

/// Marker component for the Mine boss type
#[derive(Component, Debug)]
pub struct MineBoss;

/// Component for boss skills and abilities
#[derive(Component, Debug)]
pub struct BossSkills {
    pub dash_cooldown: Timer,
    pub is_dashing: bool,
    pub dash_target: Vec3,
    pub dash_speed: f32,
    pub dash_damage: f32,
    pub dash_duration: Timer,
}

impl Default for BossSkills {
    fn default() -> Self {
        Self {
            dash_cooldown: Timer::from_seconds(3.0, TimerMode::Repeating),
            is_dashing: false,
            dash_target: Vec3::ZERO,
            dash_speed: 600.0,
            dash_damage: 25.0,
            dash_duration: Timer::from_seconds(0.8, TimerMode::Once),
        }
    }
}

impl BossSkills {
    pub fn new(cooldown_seconds: f32, dash_speed: f32, damage: f32) -> Self {
        Self {
            dash_cooldown: Timer::from_seconds(cooldown_seconds, TimerMode::Repeating),
            is_dashing: false,
            dash_target: Vec3::ZERO,
            dash_speed,
            dash_damage: damage,
            dash_duration: Timer::from_seconds(0.8, TimerMode::Once),
        }
    }

    pub fn can_dash(&self) -> bool {
        self.dash_cooldown.finished() && !self.is_dashing
    }

    pub fn start_dash(&mut self, target: Vec3) {
        if self.can_dash() {
            self.is_dashing = true;
            self.dash_target = target;
            self.dash_duration.reset();
            self.dash_cooldown.reset();
        }
    }

    pub fn update_dash(&mut self, delta: f32) -> bool {
        if self.is_dashing {
            self.dash_duration.tick(std::time::Duration::from_secs_f32(delta));
            if self.dash_duration.finished() {
                self.is_dashing = false;
                return true; // Dash completed
            }
        }
        false
    }
}

/// Component for rotating animation
#[derive(Component, Debug)]
pub struct RotationAnimation {
    pub speed: f32, // Radians per second
    pub enabled: bool,
}

impl Default for RotationAnimation {
    fn default() -> Self {
        Self {
            speed: 3.0, // Default rotation speed
            enabled: false,
        }
    }
}

impl RotationAnimation {
    pub fn new(speed: f32) -> Self {
        Self {
            speed,
            enabled: false,
        }
    }

    pub fn start(&mut self) {
        self.enabled = true;
    }

    pub fn stop(&mut self) {
        self.enabled = false;
    }
}

/// Bundle for Mine boss with all necessary components
#[derive(Bundle)]
pub struct MineBossBundle {
    pub boss: Boss,
    pub mine_boss: MineBoss,
    pub health: Health,
    pub speed: Speed,
    pub skills: BossSkills,
    pub rotation_animation: RotationAnimation,
    pub transform: Transform,
    pub mesh: Mesh2d,
    pub material: MeshMaterial2d<ColorMaterial>,
}

impl MineBossBundle {
    pub fn new(
        max_health: f32,
        speed: f32,
        position: Vec3,
        mesh: Handle<Mesh>,
        material: Handle<ColorMaterial>,
    ) -> Self {
        Self {
            boss: Boss,
            mine_boss: MineBoss,
            health: Health::new(max_health),
            speed: Speed::new(speed),
            skills: BossSkills::default(),
            rotation_animation: RotationAnimation::default(),
            transform: Transform::from_translation(position),
            mesh: Mesh2d(mesh),
            material: MeshMaterial2d(material),
        }
    }
}

/// Bundle for Mine boss spikes (the 8 squares around the circle)
#[derive(Bundle)]
pub struct MineSpikeBundle {
    pub transform: Transform,
    pub mesh: Mesh2d,
    pub material: MeshMaterial2d<ColorMaterial>,
}

impl MineSpikeBundle {
    pub fn new(
        position: Vec3,
        mesh: Handle<Mesh>,
        material: Handle<ColorMaterial>,
    ) -> Self {
        Self {
            transform: Transform::from_translation(position),
            mesh: Mesh2d(mesh),
            material: MeshMaterial2d(material),
        }
    }
} 