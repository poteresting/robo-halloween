use crate::frame::{Drawable, Frame, FRAME_COLS};

pub struct Score {
    pumpkins: usize,
    zombies: usize,
    time_left: String,
}

impl Score {
    pub fn new() -> Self {
        Self {
            pumpkins: 0,
            zombies: 0,
            time_left: "00:00".to_owned(),
        }
    }

    pub fn add_pumpkin(&mut self) {
        self.pumpkins += 1;
    }

    pub fn add_zombie(&mut self) {
        self.zombies += 1;
    }

    pub fn update_time_left(&mut self, time: String) {
        self.time_left = time;
    }
}

impl Drawable for Score {
    fn draw(&self, frame: &mut Frame) {
        frame[0][1] = format!("🎃 {}", self.pumpkins);
        frame[0][FRAME_COLS - 6] = format!("🧟 {}", self.zombies);
        frame[0][FRAME_COLS / 2 - 4] = format!("⏰ {}", self.time_left);

        for col in 0..FRAME_COLS - 1 {
            frame[1][col] = "❇️".to_owned();
        }
    }
}
