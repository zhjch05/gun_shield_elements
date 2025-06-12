use bevy::prelude::*;
use crate::components::{MenuButton};
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