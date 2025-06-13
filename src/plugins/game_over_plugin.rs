use bevy::prelude::*;
use crate::states::AppState;
use crate::components::{GameOverUI, GameOverButton};
use crate::systems::{cleanup_ui, create_button_with_component, button_hover_system};
use crate::constants::AppColors;

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::GameOver), setup_game_over_screen)
            .add_systems(
                Update,
                (
                    handle_game_over_buttons,
                    button_hover_system,
                ).run_if(in_state(AppState::GameOver)),
            )
            .add_systems(OnExit(AppState::GameOver), cleanup_ui::<GameOverUI>);
    }
}

/// System to setup the game over screen UI
fn setup_game_over_screen(mut commands: Commands) {
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
            BackgroundColor(AppColors::BACKGROUND),
            GameOverUI,
        ))
        .with_children(|parent| {
            // Game Over Title
            parent.spawn((
                Text::new("GAME OVER"),
                TextFont {
                    font_size: 80.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.2, 0.2)), // Red color for dramatic effect
                Node {
                    margin: UiRect::bottom(Val::Px(50.0)),
                    ..default()
                },
            ));

            // Menu buttons container
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                })
                .with_children(|parent| {
                    // Restart Game button
                    create_button_with_component(parent, "Restart Game", 32.0, GameOverButton::RestartGame);

                    // Back to Menu button
                    create_button_with_component(parent, "Back to Menu", 32.0, GameOverButton::BackToMenu);

                    // Exit to Desktop button
                    create_button_with_component(parent, "Exit to Desktop", 32.0, GameOverButton::ExitToDesktop);
                });
        });
}

/// System to handle game over button clicks
fn handle_game_over_buttons(
    mut interaction_query: Query<
        (&Interaction, &GameOverButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<AppState>>,
    mut exit: EventWriter<AppExit>,
) {
    for (interaction, button) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            match button {
                GameOverButton::RestartGame => {
                    info!("Restarting game");
                    next_state.set(AppState::Game);
                }
                GameOverButton::BackToMenu => {
                    info!("Returning to main menu");
                    next_state.set(AppState::MainMenu);
                }
                GameOverButton::ExitToDesktop => {
                    info!("Exiting to desktop");
                    exit.write(AppExit::Success);
                }
            }
        }
    }
} 