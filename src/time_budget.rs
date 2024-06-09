use std::time::{Duration, Instant};

pub struct TimeBudget {
    expiration: Instant,
}

impl TimeBudget {
    pub fn new(budget: Duration) -> Self {
        Self {
            expiration: Instant::now() + budget,
        }
    }

    pub fn is_expired(&self) -> bool {
        Instant::now() >= self.expiration
    }
}
