use bevy::prelude::*;
use crate::components::{Player, Energy, EnergyBarUI, EnergyBarFill};

/// System to spawn energy bar UI
pub fn spawn_energy_bar(mut commands: Commands) {
    // Energy bar container - positioned below health bar
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(50.0), // Below health bar (health bar at 20px, this at 50px)
            left: Val::Px(20.0),
            width: Val::Px(200.0),
            height: Val::Px(15.0), // Slightly smaller than health bar
            border: UiRect::all(Val::Px(2.0)),
            ..default()
        },
        BackgroundColor(Color::srgb(0.2, 0.2, 0.3)), // Slightly blue-tinted dark background
        BorderColor(Color::srgb(0.4, 0.4, 0.6)), // Blue-tinted border
        BorderRadius::all(Val::Px(4.0)),
        ZIndex(100), // On top
        EnergyBarUI,
    )).with_children(|parent| {
        // Energy bar fill
        parent.spawn((
            Node {
                width: Val::Percent(100.0), // Will be updated based on energy percentage
                height: Val::Percent(100.0),
                ..default()
            },
            BackgroundColor(Color::srgb(0.2, 0.6, 0.8)), // Blue energy bar
            BorderRadius::all(Val::Px(2.0)),
            EnergyBarFill,
        ));
    });
}

/// System to update energy bar based on player energy
pub fn update_energy_bar(
    player_query: Query<&Energy, With<Player>>,
    mut energy_fill_query: Query<&mut Node, With<EnergyBarFill>>,
) {
    if let Ok(energy) = player_query.single() {
        if let Ok(mut node) = energy_fill_query.single_mut() {
            let energy_percentage = energy.percentage();
            node.width = Val::Percent(energy_percentage * 100.0);
        }
    }
}

/// System to update energy bar color based on energy percentage
pub fn update_energy_bar_color(
    player_query: Query<&Energy, With<Player>>,
    mut energy_fill_query: Query<&mut BackgroundColor, With<EnergyBarFill>>,
) {
    if let Ok(energy) = player_query.single() {
        if let Ok(mut bg_color) = energy_fill_query.single_mut() {
            let energy_percentage = energy.percentage();
            
            // Change color from red to yellow to blue based on energy
            if energy_percentage > 0.6 {
                // Full energy - bright blue
                let blue_factor = (energy_percentage - 0.6) / 0.4;
                *bg_color = BackgroundColor(Color::srgb(
                    0.2 * (1.0 - blue_factor), 
                    0.6 + 0.2 * blue_factor, 
                    0.8
                ));
            } else if energy_percentage > 0.3 {
                // Medium energy - cyan to blue
                let cyan_factor = (energy_percentage - 0.3) / 0.3;
                *bg_color = BackgroundColor(Color::srgb(
                    0.2 + 0.4 * (1.0 - cyan_factor), 
                    0.6, 
                    0.8
                ));
            } else {
                // Low energy - yellow to orange
                *bg_color = BackgroundColor(Color::srgb(0.6, 0.4, 0.2));
            }
        }
    }
} 