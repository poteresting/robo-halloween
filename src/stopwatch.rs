use std::time::Instant;

pub struct Stopwatch {
    start: Instant,
    time: u64,
}

impl Stopwatch {
    pub fn new(seconds: u64) -> Self {
        Self {
            start: Instant::now(),
            time: seconds,
        }
    }

    pub fn elapsed(&self) -> u64 {
        self.start.elapsed().as_secs()
    }

    pub fn expired(&self) -> bool {
        self.start.elapsed().as_secs() >= self.time
    }

    pub fn time_remaining(&self) -> String {
        let remaining_time = self.time - self.elapsed();
        let minutes = remaining_time / 60;
        let seconds = remaining_time % 60;
        format!("{:02}:{:02}", minutes, seconds)
    }
}
