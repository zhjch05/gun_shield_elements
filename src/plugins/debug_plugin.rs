use bevy::prelude::*;
use crate::states::AppState;
use crate::components::DebugUI;
use crate::systems::cleanup_ui;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Debug), setup_debug_screen)
            .add_systems(OnExit(AppState::Debug), cleanup_ui::<DebugUI>);
    }
}

/// System to setup the debug screen UI (completely empty)
fn setup_debug_screen(mut commands: Commands) {
    // Empty debug screen - no UI elements
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        BackgroundColor(Color::srgb(0.1, 0.05, 0.05)),
        DebugUI,
    ));
}
