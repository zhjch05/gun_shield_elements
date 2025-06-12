use bevy::prelude::*;
use crate::states::AppState;
use crate::components::{DebugUI, DebugButton};
use crate::systems::{handle_debug_buttons, button_hover_system, cleanup_ui, create_button_with_component};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Debug), setup_debug_screen)
            .add_systems(
                Update,
                (handle_debug_buttons, button_hover_system).run_if(in_state(AppState::Debug)),
            )
            .add_systems(OnExit(AppState::Debug), cleanup_ui::<DebugUI>);
    }
}

/// System to setup the debug screen UI (empty for now)
fn setup_debug_screen(mut commands: Commands) {
    // Root UI container
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.1, 0.05, 0.05)),
            DebugUI,
        ))
        .with_children(|parent| {
            // Placeholder text
            parent.spawn((
                Text::new("Debug Screen\n(Empty for now)"),
                TextFont {
                    font_size: 40.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    margin: UiRect::bottom(Val::Px(50.0)),
                    ..default()
                },
                TextLayout::new_with_justify(JustifyText::Center),
            ));

            // Back to menu button
            create_button_with_component(parent, "Back to Menu", 24.0, DebugButton::BackToMenu);
        });
}
