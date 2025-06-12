use bevy::prelude::*;

/// Resource to track pause state and timer
#[derive(Resource, Default)]
pub struct PauseState {
    pub is_paused: bool,
    pub pause_timer: Timer,
}

impl PauseState {
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
