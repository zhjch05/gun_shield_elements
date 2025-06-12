use bevy::prelude::*;
use crate::states::AppState;
use crate::components::{DebugUI, PauseOverlayUI, Player};
use crate::systems::{
    cleanup_ui, handle_pause_input, update_pause_timer,
    spawn_pause_overlay, despawn_pause_overlay, handle_pause_buttons, button_hover_system,
    reset_pause_state, spawn_player, player_movement, camera_follow_player, cleanup_player, cleanup_debug_entities
};

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
                    update_debug_info,
                ).run_if(in_state(AppState::Debug)),
            )
            .add_systems(OnExit(AppState::Debug), (
                cleanup_ui::<DebugUI>,
                cleanup_ui::<PauseOverlayUI>,
                cleanup_player,
                cleanup_debug_entities,
                reset_pause_state,
            ));
    }
}

#[derive(Component)]
struct DebugInfoText;

/// System to setup the debug screen UI
fn setup_debug_screen(mut commands: Commands) {
    // Invisible background UI node to not interfere with game entities
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            ..default()
        },
        BackgroundColor(Color::NONE), // Completely transparent
        ZIndex(-1), // Behind game entities
        DebugUI,
    ));

    // Debug info display
    commands.spawn((
        Text::new("Debug Mode\nUse WASD to move\nESC to pause\nPlayer: Bright cyan circle\nCenter marker: Red circle at (0,0)"),
        TextFont {
            font_size: 20.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
        ZIndex(100), // On top
        DebugUI,
        DebugInfoText,
    ));

    info!("Debug screen setup complete with transparent background");
}

/// System to update debug information
fn update_debug_info(
    player_query: Query<&Transform, With<Player>>,
    mut debug_text_query: Query<&mut Text, With<DebugInfoText>>,
) {
    if let Ok(player_transform) = player_query.single() {
        if let Ok(mut text) = debug_text_query.single_mut() {
            let pos = player_transform.translation;
            **text = format!(
                "Debug Mode\nUse WASD to move\nESC to pause\nPlayer: Bright cyan circle\nCenter marker: Red circle at (0,0)\nPlayer Position: ({:.1}, {:.1})",
                pos.x, pos.y
            );
        }
    }
}
