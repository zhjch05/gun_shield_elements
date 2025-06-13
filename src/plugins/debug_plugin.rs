use bevy::prelude::*;
use crate::states::AppState;
use crate::components::{DebugUI, PauseOverlayUI, Player, MineBoss, BossSkills, HealthBarUI, EnergyBarUI};
use crate::systems::{
    cleanup_ui, handle_pause_input, update_pause_timer,
    spawn_pause_overlay, despawn_pause_overlay, handle_pause_buttons, button_hover_system,
    reset_pause_state, spawn_player, player_movement, player_face_mouse, camera_follow_player, cleanup_player, cleanup_debug_entities,
    handle_shield_input, animate_shield, update_shield_mesh, spawn_mine_boss, cleanup_boss_entities,
    mine_boss_ai, boss_dash_movement, boss_rotation_animation, boss_player_collision, boss_collision_damage,
    spawn_health_bar, update_health_bar, update_health_bar_color, check_player_death,
    spawn_energy_bar, update_energy_bar, update_energy_bar_color, manage_player_invulnerability
};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Debug), (setup_debug_screen, spawn_player, spawn_mine_boss, spawn_health_bar, spawn_energy_bar))
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
                ).run_if(in_state(AppState::Debug)),
            )
            .add_systems(
                Update,
                (
                    // Player systems - use chain to ensure proper ordering
                    player_movement,
                    manage_player_invulnerability,
                    player_face_mouse,
                    handle_shield_input,
                    animate_shield,
                    update_shield_mesh.after(animate_shield),
                    camera_follow_player,
                ).chain().run_if(in_state(AppState::Debug)),
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
                ).run_if(in_state(AppState::Debug)),
            )
            .add_systems(
                Update,
                (
                    // Health and game state systems
                    update_health_bar,
                    update_health_bar_color,
                    update_energy_bar,
                    update_energy_bar_color,
                    check_player_death,
                    update_debug_info,
                ).run_if(in_state(AppState::Debug)),
            )
            .add_systems(OnExit(AppState::Debug), (
                cleanup_ui::<DebugUI>,
                cleanup_ui::<PauseOverlayUI>,
                cleanup_ui::<HealthBarUI>,
                cleanup_ui::<EnergyBarUI>,
                cleanup_player,
                cleanup_boss_entities,
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
        Text::new("Debug Mode\nUse WASD to move\nMove mouse to aim\nRight click to activate shield\nSpace to dash in WASD direction (first 30% has i-frames)\nESC to pause\nPlayer: White circle (rotates to face mouse)\nDirection indicator: Small white circle (hidden when shield active)\nShield: White arc that grows from indicator\nMine Boss: Orange circle with 8 brown squares (dashes at player)"),
        TextFont {
            font_size: 20.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
        ZIndex(100), // On top
        DebugUI,
        DebugInfoText,
    ));
}

/// System to update debug information
fn update_debug_info(
    player_query: Query<(&Transform, &crate::components::Energy, &crate::components::PlayerDash, &crate::components::Invulnerability), With<Player>>,
    shield_query: Query<&crate::components::Shield>,
    boss_query: Query<(&Transform, &BossSkills), With<MineBoss>>,
    mut debug_text_query: Query<&mut Text, With<DebugInfoText>>,
) {
    if let Ok((player_transform, player_energy, player_dash, player_invulnerability)) = player_query.single() {
        if let Ok(mut text) = debug_text_query.single_mut() {
            let pos = player_transform.translation;
            
            let shield_info = if let Ok(shield) = shield_query.single() {
                format!("Shield: {} ({})", 
                    if shield.is_active { "Active" } else { "Inactive" },
                    (shield.length * 100.0) as u32
                )
            } else {
                "Shield: Not found".to_string()
            };
            
            let energy_info = format!("Energy: {:.1}/{:.1} ({:.0}%)", 
                player_energy.current, player_energy.max, player_energy.percentage() * 100.0);
            
            let dash_info = format!("Dash: {}{}", 
                if player_dash.is_dashing { "Active" } else { "Ready" },
                if player_invulnerability.is_active() { " (I-FRAMES)" } else { "" });
            
            let boss_info = if let Ok((boss_transform, boss_skills)) = boss_query.single() {
                let boss_pos = boss_transform.translation;
                let distance_to_boss = pos.distance(boss_pos);
                format!("Boss Position: ({:.1}, {:.1})\nDistance to Boss: {:.1}\nBoss Status: {}\nDash Cooldown: {:.1}s",
                    boss_pos.x, boss_pos.y, distance_to_boss,
                    if boss_skills.is_dashing { "Dashing" } else { "Idle" },
                    boss_skills.dash_cooldown.remaining_secs()
                )
            } else {
                "Boss: Not found".to_string()
            };
            
            **text = format!(
                "Debug Mode\nUse WASD to move\nMove mouse to aim\nRight click to activate shield\nSpace to dash in WASD direction (first 30% has i-frames)\nESC to pause\n{}\n{}\n{}\n{}",
                shield_info, energy_info, dash_info, boss_info
            );
        }
    }
}
