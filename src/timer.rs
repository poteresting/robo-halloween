pub struct Timer {
    start_value: i64,
    counter: i64,
}

impl Timer {
    pub fn new(millseconds: i64) -> Self {
        Self {
            counter: millseconds,
            start_value: millseconds,
        }
    }

    pub fn decrease_timer(&mut self, ms: i64) {
        if self.counter > 0 {
            self.counter -= ms;
        }
    }

    pub fn ready(&self) -> bool {
        self.counter <= 0
    }

    pub fn reset(&mut self) {
        self.counter = self.start_value;
    }
}
