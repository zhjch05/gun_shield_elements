use bevy::prelude::*;

/// Component that marks an entity as bounded by game world limits
#[derive(Component, Debug)]
pub struct BoundedMovement;

/// Component for the visual boundary system
#[derive(Component, Debug)]
pub struct BoundaryVisual; 