use bevy::prelude::*;

/// Boundary constants for the game world
pub struct GameBoundaries;

impl GameBoundaries {
    /// Half-width of the game world (full width = 2 * HALF_WIDTH)
    pub const HALF_WIDTH: f32 = 3200.0;
    
    /// Half-height of the game world (full height = 2 * HALF_HEIGHT)
    pub const HALF_HEIGHT: f32 = 1800.0;
    
    /// Get the full width of the game world
    pub fn width() -> f32 {
        Self::HALF_WIDTH * 2.0
    }
    
    /// Get the full height of the game world
    pub fn height() -> f32 {
        Self::HALF_HEIGHT * 2.0
    }
    
    /// Get the minimum X coordinate
    pub fn min_x() -> f32 {
        -Self::HALF_WIDTH
    }
    
    /// Get the maximum X coordinate
    pub fn max_x() -> f32 {
        Self::HALF_WIDTH
    }
    
    /// Get the minimum Y coordinate
    pub fn min_y() -> f32 {
        -Self::HALF_HEIGHT
    }
    
    /// Get the maximum Y coordinate
    pub fn max_y() -> f32 {
        Self::HALF_HEIGHT
    }
    
    /// Clamp a position to stay within boundaries
    pub fn clamp_position(position: Vec3) -> Vec3 {
        Vec3::new(
            position.x.clamp(Self::min_x(), Self::max_x()),
            position.y.clamp(Self::min_y(), Self::max_y()),
            position.z
        )
    }
    
    /// Check if a position is within boundaries
    pub fn is_within_bounds(position: Vec3) -> bool {
        position.x >= Self::min_x() && position.x <= Self::max_x() &&
        position.y >= Self::min_y() && position.y <= Self::max_y()
    }
} 