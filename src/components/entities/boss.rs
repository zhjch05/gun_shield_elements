use bevy::prelude::*;
use crate::components::attributes::{Health, Speed, CollisionDamage, Collider};
use crate::components::boundary::BoundedMovement;

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
    pub dash_start_position: Vec3, // Track where the dash started
    pub dash_speed: f32,
    pub dash_damage: f32,
    pub has_hit_player: bool, // Track if we've already hit the player during this dash
    pub dash_distance: f32, // Distance for each dash
    pub dash_timer: Timer, // Maximum time for a dash to prevent getting stuck
    pub last_position: Vec3, // Track last position to detect if stuck
    pub stuck_frames: u32, // Count frames where boss hasn't moved significantly
}

impl Default for BossSkills {
    fn default() -> Self {
        Self {
            dash_cooldown: Timer::from_seconds(3.0, TimerMode::Repeating),
            is_dashing: false,
            dash_target: Vec3::ZERO,
            dash_start_position: Vec3::ZERO,
            dash_speed: 800.0,
            dash_damage: 25.0,
            has_hit_player: false,
            dash_distance: 800.0, // Default dash distance
            dash_timer: Timer::from_seconds(2.0, TimerMode::Once), // Max 2 seconds per dash
            last_position: Vec3::ZERO,
            stuck_frames: 0,
        }
    }
}

impl BossSkills {
    pub fn can_dash(&self) -> bool {
        self.dash_cooldown.finished() && !self.is_dashing
    }

    pub fn start_dash(&mut self, target: Vec3, start_position: Vec3) {
        if self.can_dash() {
            self.is_dashing = true;
            self.dash_target = target;
            self.dash_start_position = start_position;
            self.dash_cooldown.reset();
            self.has_hit_player = false; // Reset hit tracking for new dash
            self.dash_timer.reset(); // Reset dash timer
            self.last_position = start_position;
            self.stuck_frames = 0;
        }
    }

    pub fn update_dash(&mut self, current_position: Vec3, delta_time: f32) -> bool {
        if self.is_dashing {
            // Update dash timer
            self.dash_timer.tick(std::time::Duration::from_secs_f32(delta_time));
            
            // Check if we've traveled the full dash distance
            let distance_traveled = self.dash_start_position.distance(current_position);
            
            // Check if we're stuck (not moving much)
            let movement_this_frame = self.last_position.distance(current_position);
            if movement_this_frame < 1.0 { // Less than 1 unit of movement
                self.stuck_frames += 1;
            } else {
                self.stuck_frames = 0;
            }
            self.last_position = current_position;
            
            // Complete dash if any of these conditions are met:
            // 1. Traveled full distance
            // 2. Reached target (within 50 units)
            // 3. Been stuck for too many frames (likely hit boundary)
            // 4. Dash timer expired (failsafe)
            let reached_target = current_position.distance(self.dash_target) < 50.0;
            let stuck_too_long = self.stuck_frames >= 10; // 10 frames without significant movement
            let time_expired = self.dash_timer.finished();
            
            if distance_traveled >= self.dash_distance || reached_target || stuck_too_long || time_expired {
                self.is_dashing = false;
                return true; // Dash completed
            }
        }
        false
    }
    
    pub fn can_hit_player(&self) -> bool {
        self.is_dashing && !self.has_hit_player
    }
    
    pub fn mark_player_hit(&mut self) {
        self.has_hit_player = true;
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
    pub collision_damage: CollisionDamage,
    pub collider: Collider,
    pub rotation_animation: RotationAnimation,
    pub bounded_movement: BoundedMovement,
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
            collision_damage: CollisionDamage::new(20.0, 0.5), // 20 DPS, damage every 0.5 seconds
            collider: Collider::new(30.0), // Boss radius
            rotation_animation: RotationAnimation::default(),
            bounded_movement: BoundedMovement,
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