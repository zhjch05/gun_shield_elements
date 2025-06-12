use bevy::prelude::*;

/// Component for entity movement speed
#[derive(Component, Debug, Clone)]
pub struct Speed {
    pub value: f32,
}

impl Speed {
    pub fn new(speed: f32) -> Self {
        Self { value: speed }
    }
    
    pub fn set(&mut self, speed: f32) {
        self.value = speed;
    }
    
    pub fn modify(&mut self, multiplier: f32) {
        self.value *= multiplier;
    }
    
    pub fn add(&mut self, bonus: f32) {
        self.value += bonus;
    }
}
