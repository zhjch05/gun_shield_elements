use bevy::prelude::*;

mod states;
mod plugins;
mod components;
mod systems;
mod resources;

use states::AppState;
use plugins::{MenuPlugin, GamePlugin, DebugPlugin};
use resources::PauseState;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Gun, Shield & Elements".into(),
                mode: bevy::window::WindowMode::BorderlessFullscreen(bevy::window::MonitorSelection::Primary),
                ..default()
            }),
            ..default()
        }))
        .init_state::<AppState>()
        .init_resource::<PauseState>()
        .add_plugins((
            MenuPlugin,
            GamePlugin,
            DebugPlugin,
        ))
        .run();
}