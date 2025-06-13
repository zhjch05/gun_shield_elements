use bevy::prelude::*;

/// Represents the different states of the application
#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    Debug,
    GameOver,
}
