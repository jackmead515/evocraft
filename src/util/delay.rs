#[derive(Debug, Clone)]
pub struct Delay {
    start_time: f64,
    duration: f64,
}

impl Delay {
    pub fn new(start_time: f64, duration: f64) -> Self {
        return Delay { start_time, duration };
    }

    pub fn is_complete(&self, now: f64) -> bool {
        return now - self.start_time > self.duration;
    }
}