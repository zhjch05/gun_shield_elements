use bevy::prelude::*;
use crate::components::{MenuButton, DebugButton, GameButton};
use crate::states::AppState;

/// System to handle main menu button interactions
pub fn handle_menu_buttons(
    mut interaction_query: Query<
        (&Interaction, &MenuButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<AppState>>,
    mut exit: EventWriter<AppExit>,
) {
    for (interaction, menu_button) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button {
                MenuButton::StartGame => {
                    next_state.set(AppState::Game);
                }
                MenuButton::DebugMode => {
                    next_state.set(AppState::Debug);
                }
                MenuButton::ExitToDesktop => {
                    exit.write(AppExit::Success);
                }
            }
        }
    }
}

/// System to handle debug screen button interactions
pub fn handle_debug_buttons(
    mut interaction_query: Query<
        (&Interaction, &DebugButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for (interaction, debug_button) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            match debug_button {
                DebugButton::BackToMenu => {
                    next_state.set(AppState::MainMenu);
                }
            }
        }
    }
}

/// System to handle game screen button interactions
pub fn handle_game_buttons(
    mut interaction_query: Query<
        (&Interaction, &GameButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for (interaction, game_button) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            match game_button {
                GameButton::BackToMenu => {
                    next_state.set(AppState::MainMenu);
                }
            }
        }
    }
}
