use bevy::prelude::*;

/// Marker component for the main menu UI
#[derive(Component)]
pub struct MainMenuUI;

/// Marker component for game UI
#[derive(Component)]
pub struct GameUI;

/// Marker component for debug UI
#[derive(Component)]
pub struct DebugUI;

/// Component to identify different menu buttons
#[derive(Component)]
pub enum MenuButton {
    StartGame,
    DebugMode,
    ExitToDesktop,
}

/// Marker component for pause overlay UI
#[derive(Component)]
pub struct PauseOverlayUI;

/// Component to identify different pause menu buttons
#[derive(Component)]
pub enum PauseButton {
    Resume,
    BackToMenu,
    ExitToDesktop,
}