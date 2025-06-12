use bevy::prelude::*;
use crate::states::AppState;
use crate::components::{MainMenuUI, MenuButton};
use crate::systems::{handle_menu_buttons, button_hover_system, cleanup_ui, create_button_with_component};
use crate::constants::AppColors;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_camera)
            .add_systems(OnEnter(AppState::MainMenu), setup_main_menu)
            .add_systems(
                Update,
                (handle_menu_buttons, button_hover_system).run_if(in_state(AppState::MainMenu)),
            )
            .add_systems(OnExit(AppState::MainMenu), cleanup_ui::<MainMenuUI>);
    }
}

/// System to setup the camera
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

/// System to setup the main menu UI
fn setup_main_menu(mut commands: Commands) {
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
            MainMenuUI,
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("Gun, Shield & Elements"),
                TextFont {
                    font_size: 60.0,
                    ..default()
                },
                TextColor(Color::WHITE),
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
                    // Start Game button
                    create_button_with_component(parent, "Start Game", 32.0, MenuButton::StartGame);

                    // Debug Mode button
                    create_button_with_component(parent, "Debug Mode", 32.0, MenuButton::DebugMode);

                    // Exit to Desktop button
                    create_button_with_component(parent, "Exit to Desktop", 32.0, MenuButton::ExitToDesktop);
                });
        });
}
