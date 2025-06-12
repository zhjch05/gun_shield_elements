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
}
