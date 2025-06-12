use bevy::prelude::*;

mod states;
mod plugins;
mod components;
mod systems;

use states::AppState;
use plugins::{MenuPlugin, GamePlugin, DebugPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Gun, Shield & Elements".into(),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }))
        .init_state::<AppState>()
        .add_plugins((
            MenuPlugin,
            GamePlugin,
            DebugPlugin,
        ))
        .run();
}