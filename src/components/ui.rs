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

/// Marker component for debug-specific entities that should be cleaned up on exit
#[derive(Component)]
pub struct DebugEntity;

/// Marker component for health bar UI
#[derive(Component)]
pub struct HealthBarUI;

/// Component for the health bar fill element
#[derive(Component)]
pub struct HealthBarFill;

/// Marker component for energy bar UI
#[derive(Component)]
pub struct EnergyBarUI;

/// Component for the energy bar fill element
#[derive(Component)]
pub struct EnergyBarFill;

/// Marker component for boss health bar UI
#[derive(Component)]
pub struct BossHealthBarUI;

/// Component for the boss health bar fill element
#[derive(Component)]
pub struct BossHealthBarFill;

/// Marker component for game over UI
#[derive(Component)]
pub struct GameOverUI;

/// Component to identify different game over buttons
#[derive(Component)]
pub enum GameOverButton {
    RestartGame,
    BackToMenu,
    ExitToDesktop,
}