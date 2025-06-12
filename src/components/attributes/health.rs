use bevy::prelude::*;

/// Component for entity health
#[derive(Component, Debug, Clone)]
pub struct Health;

impl Health {
    pub fn new(_max_health: f32) -> Self {
        Self
    }
}
