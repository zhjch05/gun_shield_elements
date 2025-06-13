use bevy::prelude::*;

/// Component for circular collider with radius
#[derive(Component, Debug, Clone)]
pub struct Collider {
    pub radius: f32,
}

impl Collider {
    pub fn new(radius: f32) -> Self {
        Self { radius }
    }
} 