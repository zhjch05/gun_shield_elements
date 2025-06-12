use bevy::prelude::*;
use crate::states::AppState;
use crate::components::{GameUI, GameButton};
use crate::systems::{handle_game_buttons, button_hover_system, cleanup_ui, create_button_with_component};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Game), setup_game_screen)
            .add_systems(
                Update,
                (handle_game_buttons, button_hover_system).run_if(in_state(AppState::Game)),
            )
            .add_systems(OnExit(AppState::Game), cleanup_ui::<GameUI>);
    }
}

/// System to setup the game screen UI (empty for now)
fn setup_game_screen(mut commands: Commands) {
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
            BackgroundColor(Color::srgb(0.05, 0.1, 0.05)),
            GameUI,
        ))
        .with_children(|parent| {
            // Placeholder text
            parent.spawn((
                Text::new("Game Screen\n(Empty for now)"),
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
            create_button_with_component(parent, "Back to Menu", 24.0, GameButton::BackToMenu);
        });
}
