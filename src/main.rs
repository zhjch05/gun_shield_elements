use bevy::prelude::*;
use bevy::window::{WindowMode, MonitorSelection};

mod states;
mod plugins;
mod components;
mod systems;
mod resources;
mod constants;

use states::AppState;
use plugins::{MenuPlugin, GamePlugin, DebugPlugin, GameOverPlugin};
use resources::PauseState;

fn main() {
    let mut app = App::new();
    
    // Configure window based on platform
    #[cfg(target_arch = "wasm32")]
    let window_plugin = WindowPlugin {
        primary_window: Some(Window {
            fit_canvas_to_parent: true,
            prevent_default_event_handling: false,
            ..default()
        }),
        ..default()
    };
    
    #[cfg(not(target_arch = "wasm32"))]
    let window_plugin = WindowPlugin {
        primary_window: Some(Window {
            mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
            ..default()
        }),
        ..default()
    };
    
    app.add_plugins(DefaultPlugins.set(window_plugin))
        .init_state::<AppState>()
        .init_resource::<PauseState>()
        .add_plugins((
            MenuPlugin,
            GamePlugin,
            DebugPlugin,
            GameOverPlugin,
        ))
        .run();
}