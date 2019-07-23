// Profiler

use crate::dom::timestamp;
use std::collections::VecDeque;

pub struct Profiler {
    deltas: VecDeque<f64>,
    start_time: f64,
}

impl Profiler {
    pub fn new() -> Self {
        Self {
            deltas: VecDeque::new(),
            start_time: timestamp(),
        }
    }

    pub fn start(&mut self) {
        self.start_time = timestamp();
    }

    pub fn stop(&mut self) {
        let now = timestamp();
        let delta = now - self.start_time;

        self.deltas.push_back(delta);

        if self.deltas.len() > 100 {
            self.deltas.pop_front().unwrap();
        }
    }

    pub fn mean(&mut self) -> f64 {
        let sum: f64 = self.deltas.iter().sum();
        sum / self.deltas.len() as f64
    }
}
