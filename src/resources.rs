use bevy::prelude::*;

/// Resource to track pause state and timer
#[derive(Resource, Default)]
pub struct PauseState {
    pub is_paused: bool,
    pub pause_timer: Timer,
}

impl PauseState {
    pub fn new() -> Self {
        Self {
            is_paused: false,
            pause_timer: Timer::from_seconds(0.0, TimerMode::Once),
        }
    }

    pub fn pause(&mut self) {
        self.is_paused = true;
        self.pause_timer.reset();
    }

    pub fn resume(&mut self) {
        self.is_paused = false;
    }

    pub fn toggle(&mut self) {
        if self.is_paused {
            self.resume();
        } else {
            self.pause();
        }
    }
}
