use bevy::prelude::*;
use crate::states::AppState;
use crate::components::{GameUI, PauseOverlayUI, HealthBarUI};
use crate::systems::{
    cleanup_ui, handle_pause_input, update_pause_timer,
    spawn_pause_overlay, despawn_pause_overlay, handle_pause_buttons, button_hover_system,
    reset_pause_state, spawn_health_bar, update_health_bar, update_health_bar_color,
    check_player_death, spawn_player, cleanup_player, cleanup_debug_entities,
    player_movement, player_face_mouse, camera_follow_player,
    spawn_mine_boss, cleanup_boss_entities,
    mine_boss_ai, boss_dash_movement, boss_rotation_animation, boss_player_collision, boss_collision_damage
};
use crate::constants::AppColors;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Game), (setup_game_screen, spawn_health_bar, spawn_player, spawn_mine_boss))
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
                    player_face_mouse,
                    camera_follow_player,
                    mine_boss_ai,
                    boss_dash_movement,
                    boss_rotation_animation,
                    boss_player_collision,
                    update_health_bar,
                    update_health_bar_color,
                    check_player_death,
                ).run_if(in_state(AppState::Game)),
            )
            .add_systems(OnExit(AppState::Game), (
                cleanup_ui::<GameUI>,
                cleanup_ui::<PauseOverlayUI>,
                cleanup_ui::<HealthBarUI>,
                cleanup_player,
                cleanup_boss_entities,
                cleanup_debug_entities,
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
        BackgroundColor(AppColors::BACKGROUND),
        GameUI,
    ));
}
