use bevy::prelude::*;

/// Component for energy/stamina system
#[derive(Component, Debug, Clone)]
pub struct Energy {
    pub current: f32,
    pub max: f32,
    pub recharge_rate: f32, // Energy per second recharge rate
}

impl Energy {
    pub fn new(max_energy: f32, recharge_rate: f32) -> Self {
        Self {
            current: max_energy,
            max: max_energy,
            recharge_rate,
        }
    }

    pub fn consume(&mut self, amount: f32) -> bool {
        if self.current >= amount {
            self.current -= amount;
            true
        } else {
            false
        }
    }

    pub fn recharge(&mut self, delta: f32) {
        self.current = (self.current + self.recharge_rate * delta).min(self.max);
    }

    pub fn can_consume(&self, amount: f32) -> bool {
        self.current >= amount
    }

    pub fn is_full(&self) -> bool {
        self.current >= self.max
    }

    pub fn percentage(&self) -> f32 {
        if self.max > 0.0 {
            self.current / self.max
        } else {
            0.0
        }
    }
} 