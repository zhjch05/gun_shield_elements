use bevy::prelude::*;

/// Color constants for consistent theming across the application
pub struct AppColors;

impl AppColors {
    /// Mid-gray background color used for all screens
    pub const BACKGROUND: Color = Color::srgb(0.4, 0.4, 0.4);
    
    /// Button colors
    pub const BUTTON_NORMAL: Color = Color::srgb(0.2, 0.2, 0.2);
    pub const BUTTON_HOVERED: Color = Color::srgb(0.3, 0.3, 0.3);
    pub const BUTTON_PRESSED: Color = Color::srgb(0.1, 0.1, 0.1);
    pub const BUTTON_BORDER: Color = Color::srgb(0.4, 0.4, 0.4);
    
    /// Text colors
    pub const TEXT_PRIMARY: Color = Color::WHITE;
    
    /// Overlay colors
    pub const PAUSE_OVERLAY: Color = Color::srgba(0.0, 0.0, 0.0, 0.8);
}
