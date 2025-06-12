use bevy::prelude::*;
use crate::states::AppState;
use crate::components::{GameUI, PauseOverlayUI};
use crate::resources::PauseState;
use crate::systems::{
    cleanup_ui, handle_pause_input, update_pause_timer,
    spawn_pause_overlay, despawn_pause_overlay, handle_pause_buttons, button_hover_system,
    reset_pause_state
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<PauseState>()
            .add_systems(OnEnter(AppState::Game), setup_game_screen)
            .add_systems(
                Update,
                (
                    handle_pause_input,
                    update_pause_timer,
                    spawn_pause_overlay,
                    despawn_pause_overlay,
                    handle_pause_buttons,
                    button_hover_system,
                ).run_if(in_state(AppState::Game)),
            )
            .add_systems(OnExit(AppState::Game), (
                cleanup_ui::<GameUI>,
                cleanup_ui::<PauseOverlayUI>,
                reset_pause_state,
            ));
    }
}

/// System to setup the game screen UI (completely empty)
fn setup_game_screen(mut commands: Commands) {
    // Empty game screen - no UI elements
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        BackgroundColor(Color::srgb(0.05, 0.1, 0.05)),
        GameUI,
    ));
}
