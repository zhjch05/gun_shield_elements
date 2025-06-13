use bevy::prelude::*;
use crate::components::{Player, Health, HealthBarUI, HealthBarFill};

/// System to spawn health bar UI
pub fn spawn_health_bar(mut commands: Commands) {
    // Health bar container
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(20.0),
            left: Val::Px(20.0),
            width: Val::Px(200.0),
            height: Val::Px(20.0),
            border: UiRect::all(Val::Px(2.0)),
            ..default()
        },
        BackgroundColor(Color::srgb(0.3, 0.3, 0.3)), // Dark gray background
        BorderColor(Color::WHITE),
        BorderRadius::all(Val::Px(4.0)),
        ZIndex(100), // On top
        HealthBarUI,
    )).with_children(|parent| {
        // Health bar fill
        parent.spawn((
            Node {
                width: Val::Percent(100.0), // Will be updated based on health percentage
                height: Val::Percent(100.0),
                ..default()
            },
            BackgroundColor(Color::srgb(0.8, 0.2, 0.2)), // Red health bar
            BorderRadius::all(Val::Px(2.0)),
            HealthBarFill,
        ));
    });
}

/// System to update health bar based on player health
pub fn update_health_bar(
    player_query: Query<&Health, With<Player>>,
    mut health_fill_query: Query<&mut Node, With<HealthBarFill>>,
) {
    if let Ok(health) = player_query.single() {
        if let Ok(mut node) = health_fill_query.single_mut() {
            let health_percentage = health.percentage();
            node.width = Val::Percent(health_percentage * 100.0);
            
            // Optional: Change color based on health percentage
            // This would require also querying for BackgroundColor
        }
    }
}

/// System to update health bar color based on health percentage
pub fn update_health_bar_color(
    player_query: Query<&Health, With<Player>>,
    mut health_fill_query: Query<&mut BackgroundColor, With<HealthBarFill>>,
) {
    if let Ok(health) = player_query.single() {
        if let Ok(mut bg_color) = health_fill_query.single_mut() {
            let health_percentage = health.percentage();
            
            // Change color from red to yellow to green based on health
            if health_percentage > 0.6 {
                // Green to yellow
                let green_factor = (health_percentage - 0.6) / 0.4;
                *bg_color = BackgroundColor(Color::srgb(0.8 * (1.0 - green_factor), 0.8, 0.2 * (1.0 - green_factor)));
            } else if health_percentage > 0.3 {
                // Yellow to red
                let yellow_factor = (health_percentage - 0.3) / 0.3;
                *bg_color = BackgroundColor(Color::srgb(0.8, 0.8 * yellow_factor, 0.2 * yellow_factor));
            } else {
                // Red
                *bg_color = BackgroundColor(Color::srgb(0.8, 0.2, 0.2));
            }
        }
    }
} 