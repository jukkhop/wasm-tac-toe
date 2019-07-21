// Fps counter

use crate::dom::timestamp;
use std::collections::VecDeque;

pub struct FpsCounter {
    frames: VecDeque<f64>,
    last_frame_timestamp: f64,
}

impl FpsCounter {
    pub fn new() -> Self {
        Self {
            frames: VecDeque::new(),
            last_frame_timestamp: timestamp(),
        }
    }

    pub fn tick(&mut self) -> f64 {
        // Convert the delta time since the last frame render into a measure
        // of frames per second.
        let now = timestamp();

        let delta = now - self.last_frame_timestamp;
        self.last_frame_timestamp = now;
        let fps = 1.0 / delta * 1000.0;

        // Save only the latest 100 timings.
        self.frames.push_back(fps);
        let frames_len = self.frames.len();

        if frames_len > 100 {
            self.frames.pop_front().unwrap();
        }

        let sum: f64 = self.frames.iter().sum();
        let mean = sum / frames_len as f64;

        mean
    }
}
