use bevy::prelude::*;

/// Color constants for consistent theming across the application
pub struct AppColors;

impl AppColors {
    /// Mid-gray background color used for all screens
    pub const BACKGROUND: Color = Color::srgb(0.1, 0.1, 0.15);
}
