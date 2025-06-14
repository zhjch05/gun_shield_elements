use bevy::prelude::*;
use crate::components::{PauseOverlayUI, PauseButton};
use crate::resources::PauseState;
use crate::states::AppState;
use crate::systems::create_button_with_component;


/// System to handle pause input (ESC key)
pub fn handle_pause_input(
    mut pause_state: ResMut<PauseState>,
    mut virtual_time: ResMut<Time<Virtual>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<AppState>>,
) {
    // Allow pausing in both Game and Debug states
    let can_pause = matches!(*current_state.get(), AppState::Game | AppState::Debug);
    if can_pause && keyboard_input.just_pressed(KeyCode::Escape) {
        pause_state.toggle();
        
        // Pause or unpause virtual time based on pause state
        if pause_state.is_paused {
            virtual_time.pause();
            info!("Game paused - virtual time stopped");
        } else {
            virtual_time.unpause();
            info!("Game resumed - virtual time running");
        }
    }
}

/// System to update pause timer using real time (not virtual time)
pub fn update_pause_timer(
    mut pause_state: ResMut<PauseState>,
    time: Res<Time<Real>>,
) {
    if pause_state.is_paused {
        pause_state.pause_timer.tick(time.delta());
    }
}

/// System to spawn pause overlay when paused
pub fn spawn_pause_overlay(
    mut commands: Commands,
    pause_state: Res<PauseState>,
    existing_overlay: Query<Entity, With<PauseOverlayUI>>,
) {
    if pause_state.is_paused && existing_overlay.is_empty() {
        // Create pause overlay
        commands
            .spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)), // Semi-transparent black overlay
                PauseOverlayUI,
                ZIndex(1000), // Ensure it's on top
            ))
            .with_children(|parent| {
                // Pause title
                parent.spawn((
                    Text::new("PAUSED"),
                    TextFont {
                        font_size: 80.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                    Node {
                        margin: UiRect::bottom(Val::Px(50.0)),
                        ..default()
                    },
                ));

                // Buttons container
                parent
                    .spawn(Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    })
                    .with_children(|parent| {
                        // Resume button
                        create_button_with_component(parent, "Resume Game", 32.0, PauseButton::Resume);

                        // Back to Menu button
                        create_button_with_component(parent, "Back to Menu", 32.0, PauseButton::BackToMenu);

                        // Exit to Desktop button
                        create_button_with_component(parent, "Exit to Desktop", 32.0, PauseButton::ExitToDesktop);
                    });
            });
    }
}

/// System to despawn pause overlay when not paused
pub fn despawn_pause_overlay(
    mut commands: Commands,
    pause_state: Res<PauseState>,
    overlay_query: Query<Entity, With<PauseOverlayUI>>,
) {
    if !pause_state.is_paused {
        for entity in &overlay_query {
            commands.entity(entity).despawn();
        }
    }
}

/// System to handle pause menu button interactions
pub fn handle_pause_buttons(
    mut interaction_query: Query<
        (&Interaction, &PauseButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut pause_state: ResMut<PauseState>,
    mut virtual_time: ResMut<Time<Virtual>>,
    mut next_state: ResMut<NextState<AppState>>,
    mut exit: EventWriter<AppExit>,
) {
    for (interaction, pause_button) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            match pause_button {
                PauseButton::Resume => {
                    pause_state.resume();
                    virtual_time.unpause();
                    info!("Game resumed via menu - virtual time running");
                }
                PauseButton::BackToMenu => {
                    pause_state.resume();
                    virtual_time.unpause();
                    next_state.set(AppState::MainMenu);
                    info!("Returning to main menu - virtual time running");
                }
                PauseButton::ExitToDesktop => {
                    exit.write(AppExit::Success);
                }
            }
        }
    }
}

/// System to reset pause state when exiting game
pub fn reset_pause_state(
    mut pause_state: ResMut<PauseState>,
    mut virtual_time: ResMut<Time<Virtual>>,
) {
    pause_state.resume();
    virtual_time.unpause();
    info!("Pause state reset - virtual time running");
}
