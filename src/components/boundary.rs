use bevy::prelude::*;

/// Component that marks an entity as bounded by game world limits
#[derive(Component, Debug)]
pub struct BoundedMovement;

/// Component for the visual boundary system
#[derive(Component, Debug)]
pub struct BoundaryVisual;

/// Component for edge warning UI elements
#[derive(Component, Debug)]
pub struct EdgeWarning {
    pub edge: EdgeType,
}

/// Enum to specify which edge the warning is for
#[derive(Debug, Clone, Copy)]
pub enum EdgeType {
    Top,
    Bottom,
    Left,
    Right,
} 