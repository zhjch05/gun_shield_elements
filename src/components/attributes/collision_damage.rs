use bevy::prelude::*;

/// Component for collision damage per second when enemy touches player
#[derive(Component, Debug, Clone)]
pub struct CollisionDamage {
    pub damage_per_second: f32,
    pub last_damage_time: f32, // Track when we last applied damage to prevent multiple hits per frame
    pub damage_interval: f32,  // Minimum time between damage applications (e.g., 0.5 seconds)
}

impl CollisionDamage {
    pub fn new(damage_per_second: f32, damage_interval: f32) -> Self {
        Self {
            damage_per_second,
            last_damage_time: 0.0,
            damage_interval,
        }
    }
    
    pub fn can_damage(&self, current_time: f32) -> bool {
        current_time - self.last_damage_time >= self.damage_interval
    }
    
    pub fn apply_damage(&mut self, current_time: f32) -> f32 {
        self.last_damage_time = current_time;
        self.damage_per_second * self.damage_interval
    }
} 