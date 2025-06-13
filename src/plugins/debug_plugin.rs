use bevy::prelude::*;
use crate::states::AppState;
use crate::components::{DebugUI, PauseOverlayUI, Player, MineBoss, BossSkills};
use crate::systems::{
    cleanup_ui, handle_pause_input, update_pause_timer,
    spawn_pause_overlay, despawn_pause_overlay, handle_pause_buttons, button_hover_system,
    reset_pause_state, spawn_player, player_movement, player_face_mouse, camera_follow_player, cleanup_player, cleanup_debug_entities,
    handle_shield_input, animate_shield, update_shield_mesh, spawn_mine_boss, cleanup_boss_entities,
    mine_boss_ai, boss_dash_movement, boss_rotation_animation, boss_player_collision
};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Debug), (setup_debug_screen, spawn_player, spawn_mine_boss))
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
                    handle_shield_input,
                    animate_shield,
                    update_shield_mesh.after(animate_shield),
                    camera_follow_player,
                    mine_boss_ai,
                    boss_dash_movement,
                    boss_rotation_animation,
                    boss_player_collision,
                    update_debug_info,
                ).run_if(in_state(AppState::Debug)),
            )
            .add_systems(OnExit(AppState::Debug), (
                cleanup_ui::<DebugUI>,
                cleanup_ui::<PauseOverlayUI>,
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
        Text::new("Debug Mode\nUse WASD to move\nMove mouse to aim\nRight click to activate shield\nESC to pause\nPlayer: White circle (rotates to face mouse)\nDirection indicator: Small white circle (hidden when shield active)\nShield: White arc that grows from indicator\nCenter marker: Red circle at (0,0)\nMine Boss: Orange circle with 8 brown squares (dashes at player)"),
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
    player_query: Query<&Transform, With<Player>>,
    shield_query: Query<&crate::components::Shield>,
    boss_query: Query<(&Transform, &BossSkills), With<MineBoss>>,
    mut debug_text_query: Query<&mut Text, With<DebugInfoText>>,
) {
    if let Ok(player_transform) = player_query.single() {
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
                "Debug Mode\nUse WASD to move\nMove mouse to aim\nRight click to activate shield\nESC to pause\n{}\n{}",
                shield_info, boss_info
            );
        }
    }
}
