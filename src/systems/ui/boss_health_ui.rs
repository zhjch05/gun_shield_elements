use bevy::prelude::*;
use crate::components::{Boss, Health, BossHealthBarUI, BossHealthBarFill};

/// System to spawn boss health bar UI when boss is present
pub fn spawn_boss_health_bar(
    mut commands: Commands,
    boss_query: Query<Entity, (With<Boss>, With<Health>)>,
    boss_health_bar_query: Query<Entity, With<BossHealthBarUI>>,
) {
    // Only spawn if boss exists and health bar doesn't already exist
    if !boss_query.is_empty() && boss_health_bar_query.is_empty() {
        // Boss health bar container - positioned at top center of screen
        commands.spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(40.0),
                left: Val::Percent(50.0),
                width: Val::Px(600.0), // Large width for boss health bar
                height: Val::Px(40.0), // Taller than player health bar
                border: UiRect::all(Val::Px(3.0)),
                // Center the health bar horizontally
                margin: UiRect {
                    left: Val::Px(-300.0), // Half of width to center
                    ..default()
                },
                ..default()
            },
            BackgroundColor(Color::srgb(0.1, 0.1, 0.1)), // Very dark background
            BorderColor(Color::srgb(0.8, 0.8, 0.8)), // Light gray border
            BorderRadius::all(Val::Px(6.0)),
            ZIndex(200), // Higher than other UI elements
            BossHealthBarUI,
        )).with_children(|parent| {
            // Boss health bar fill
            parent.spawn((
                Node {
                    width: Val::Percent(100.0), // Will be updated based on health percentage
                    height: Val::Percent(100.0),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.9, 0.1, 0.1)), // Bright red for boss health
                BorderRadius::all(Val::Px(3.0)),
                BossHealthBarFill,
            ));
            
            // Boss name/title text
            parent.spawn((
                Text::new("MINE BOSS"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(-25.0), // Above the health bar
                    left: Val::Percent(50.0),
                    margin: UiRect {
                        left: Val::Px(-50.0), // Approximate centering
                        ..default()
                    },
                    ..default()
                },
                ZIndex(201), // Above health bar
            ));
        });
    }
}

/// System to update boss health bar based on boss health
pub fn update_boss_health_bar(
    boss_query: Query<&Health, With<Boss>>,
    mut boss_health_fill_query: Query<&mut Node, With<BossHealthBarFill>>,
) {
    if let Ok(health) = boss_query.single() {
        if let Ok(mut node) = boss_health_fill_query.single_mut() {
            let health_percentage = health.percentage();
            node.width = Val::Percent(health_percentage * 100.0);
        }
    }
}

/// System to update boss health bar color based on health percentage
pub fn update_boss_health_bar_color(
    boss_query: Query<&Health, With<Boss>>,
    mut boss_health_fill_query: Query<&mut BackgroundColor, With<BossHealthBarFill>>,
) {
    if let Ok(health) = boss_query.single() {
        if let Ok(mut bg_color) = boss_health_fill_query.single_mut() {
            let health_percentage = health.percentage();
            
            // Boss health bar color changes: Red -> Orange -> Yellow as health decreases
            if health_percentage > 0.66 {
                // High health: Bright red
                *bg_color = BackgroundColor(Color::srgb(0.9, 0.1, 0.1));
            } else if health_percentage > 0.33 {
                // Medium health: Orange
                *bg_color = BackgroundColor(Color::srgb(0.9, 0.5, 0.1));
            } else {
                // Low health: Yellow (critical)
                *bg_color = BackgroundColor(Color::srgb(0.9, 0.8, 0.1));
            }
        }
    }
}

/// System to clean up boss health bar when boss is defeated or removed
pub fn cleanup_boss_health_bar_on_boss_death(
    mut commands: Commands,
    boss_query: Query<Entity, With<Boss>>,
    boss_health_bar_query: Query<Entity, With<BossHealthBarUI>>,
) {
    // If no boss exists but health bar does, clean it up
    if boss_query.is_empty() && !boss_health_bar_query.is_empty() {
        for entity in boss_health_bar_query.iter() {
            commands.entity(entity).despawn();
        }
    }
}

/// System to clean up boss health bar UI
pub fn cleanup_boss_health_bar(
    mut commands: Commands,
    boss_health_bar_query: Query<Entity, With<BossHealthBarUI>>,
) {
    for entity in boss_health_bar_query.iter() {
        commands.entity(entity).despawn();
    }
} 