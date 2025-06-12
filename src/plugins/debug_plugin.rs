use bevy::prelude::*;
use crate::states::AppState;
use crate::components::{DebugUI, PauseOverlayUI};
use crate::systems::{
    cleanup_ui, handle_pause_input, update_pause_timer,
    spawn_pause_overlay, despawn_pause_overlay, handle_pause_buttons, button_hover_system,
    reset_pause_state, spawn_player, player_movement, camera_follow_player, cleanup_player
};
use crate::constants::AppColors;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Debug), (setup_debug_screen, spawn_player))
            .add_systems(
                Update,
                (
                    handle_pause_input,
                    update_pause_timer,
                    spawn_pause_overlay,
                    despawn_pause_overlay,
                    handle_pause_buttons,
                    button_hover_system,
                    player_movement,
                    camera_follow_player,
                ).run_if(in_state(AppState::Debug)),
            )
            .add_systems(OnExit(AppState::Debug), (
                cleanup_ui::<DebugUI>,
                cleanup_ui::<PauseOverlayUI>,
                cleanup_player,
                reset_pause_state,
            ));
    }
}

/// System to setup the debug screen UI (completely empty)
fn setup_debug_screen(mut commands: Commands) {
    // Empty debug screen - no UI elements
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        BackgroundColor(AppColors::BACKGROUND),
        DebugUI,
    ));
}
