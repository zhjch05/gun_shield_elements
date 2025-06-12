use bevy::prelude::*;

/// Marker component for the player entity
#[derive(Component)]
pub struct Player;

/// Component for player health
#[derive(Component)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

impl Health {
    pub fn new(max_health: f32) -> Self {
        Self {
            current: max_health,
            max: max_health,
        }
    }
    
    pub fn is_alive(&self) -> bool {
        self.current > 0.0
    }
    
    pub fn take_damage(&mut self, damage: f32) {
        self.current = (self.current - damage).max(0.0);
    }
    
    pub fn heal(&mut self, amount: f32) {
        self.current = (self.current + amount).min(self.max);
    }
    
    pub fn health_percentage(&self) -> f32 {
        if self.max > 0.0 {
            self.current / self.max
        } else {
            0.0
        }
    }
}

/// Component for player movement speed
#[derive(Component)]
pub struct Speed {
    pub value: f32,
}

impl Speed {
    pub fn new(speed: f32) -> Self {
        Self { value: speed }
    }
}

/// Bundle for player stats
#[derive(Bundle)]
pub struct PlayerStatsBundle {
    pub health: Health,
    pub speed: Speed,
}

impl PlayerStatsBundle {
    pub fn new(max_health: f32, speed: f32) -> Self {
        Self {
            health: Health::new(max_health),
            speed: Speed::new(speed),
        }
    }
}
