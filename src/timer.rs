use std::time::{Duration, Instant};

/// A timer that can be used to calculate split times for ticks etc.
pub struct Timer {
    time: Instant,
}

impl Timer {
    /// Build a new timer with the time value set as current time.
    pub fn new() -> Self {
        Self {
            time: Instant::now(),
        }
    }

    /// Get the duration passed since the previous time call.
    pub fn time(&mut self) -> Duration {
        let now = Instant::now();
        let duration = now.duration_since(self.time);
        self.time = now;
        duration
    }
}
