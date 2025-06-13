use bevy::prelude::*;
use crate::components::{Player, Health};
use crate::states::AppState;

/// System to check if player has died and transition to game over state
pub fn check_player_death(
    player_query: Query<&Health, With<Player>>,
    mut next_state: ResMut<NextState<AppState>>,
    current_state: Res<State<AppState>>,
) {
    // Only check for death in Game and Debug states
    if !matches!(current_state.get(), AppState::Game | AppState::Debug) {
        return;
    }
    
    if let Ok(health) = player_query.single() {
        if !health.is_alive() {
            info!("Player has died! Transitioning to Game Over screen...");
            next_state.set(AppState::GameOver);
        }
    }
} 