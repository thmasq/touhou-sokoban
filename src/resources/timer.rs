use std::time::{Instant};


pub struct Timer {
    clock: Option<Instant>
}

impl Timer {
    pub fn new() -> Self {
        Timer { clock: Some(Instant::now()) }
    }

    pub fn start(&mut self) {
        self.clock = Some(Instant::now());
    }

    pub fn stop(&mut self) {
        self.clock = None;
    }
}

