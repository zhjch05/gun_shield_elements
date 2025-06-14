use bevy::prelude::*;
use crate::states::AppState;
use crate::components::{GameUI, PauseOverlayUI, HealthBarUI, EnergyBarUI};
use crate::systems::{
    cleanup_ui, handle_pause_input, update_pause_timer,
    spawn_pause_overlay, despawn_pause_overlay, handle_pause_buttons, button_hover_system,
    reset_pause_state, spawn_health_bar, update_health_bar, update_health_bar_color,
    spawn_energy_bar, update_energy_bar, update_energy_bar_color,
    check_player_death, spawn_player, cleanup_player, cleanup_debug_entities,
    player_movement, player_face_mouse, camera_follow_player, manage_player_invulnerability,

    spawn_mine_boss, cleanup_boss_entities,
    mine_boss_ai, boss_dash_movement, boss_rotation_animation, boss_player_collision, boss_collision_damage,
    spawn_boundary_visuals, enforce_boundaries, cleanup_boundary_visuals,
    spawn_edge_warnings, update_edge_warnings, cleanup_edge_warnings,
    weapon_firing_system, projectile_movement_system, projectile_lifetime_system, projectile_boss_collision_system, cleanup_projectiles
};


pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Game), (setup_game_screen, spawn_boundary_visuals, spawn_edge_warnings, spawn_health_bar, spawn_energy_bar, spawn_player, spawn_mine_boss))
            .add_systems(
                Update,
                (
                    // UI and pause systems
                    handle_pause_input,
                    update_pause_timer,
                    spawn_pause_overlay,
                    despawn_pause_overlay,
                    handle_pause_buttons,
                    button_hover_system,
                ).run_if(in_state(AppState::Game)),
            )
            .add_systems(
                Update,
                (
                    // Player systems - use chain for proper ordering
                    player_movement,
                    manage_player_invulnerability,
                    player_face_mouse,
                    enforce_boundaries, // Apply boundary constraints after movement
                    camera_follow_player,
                ).chain().run_if(in_state(AppState::Game)),
            )
            .add_systems(
                Update,
                (
                    // Weapon systems
                    weapon_firing_system,
                    projectile_movement_system,
                    projectile_lifetime_system,
                    projectile_boss_collision_system,
                ).run_if(in_state(AppState::Game)),
            )
            .add_systems(
                Update,
                (
                    // Boss systems
                    mine_boss_ai,
                    boss_dash_movement,
                    boss_rotation_animation,
                    boss_player_collision,
                    boss_collision_damage,
                ).run_if(in_state(AppState::Game)),
            )
            .add_systems(
                Update,
                (
                    // UI and game state systems
                    update_health_bar,
                    update_health_bar_color,
                    update_energy_bar,
                    update_energy_bar_color,
                    update_edge_warnings, // Update edge warning intensity
                    check_player_death,
                ).run_if(in_state(AppState::Game)),
            )
            .add_systems(OnExit(AppState::Game), (
                cleanup_ui::<GameUI>,
                cleanup_ui::<PauseOverlayUI>,
                cleanup_ui::<HealthBarUI>,
                cleanup_ui::<EnergyBarUI>,
                cleanup_player,
                cleanup_boss_entities,
                cleanup_debug_entities,
                cleanup_boundary_visuals,
                cleanup_edge_warnings,
                cleanup_projectiles,
                reset_pause_state,
            ));
    }
}

/// System to setup the game screen UI (completely empty)
fn setup_game_screen(mut commands: Commands) {
    // Empty game screen - no UI elements, background handled by boundary system
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        BackgroundColor(Color::NONE), // Transparent - boundary system provides background
        GameUI,
    ));
}
