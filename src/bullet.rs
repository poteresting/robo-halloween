use crate::{
    frame::{Drawable, Frame, FRAME_COLS},
    timer::Timer,
};

pub struct Bullet {
    pub row: usize,
    pub col: usize,
    display: &'static str,
    destroyed: bool,
    timer: Timer,
}

impl Bullet {
    pub fn new(row: usize, col: usize) -> Self {
        Self {
            row,
            col,
            display: "🔸",
            destroyed: false,
            timer: Timer::new(600),
        }
    }

    pub fn update(&mut self) {
        self.timer.decrease_timer(20);

        if self.col < FRAME_COLS - 1 && self.timer.ready() && !self.destroyed {
            self.col += 1;
            self.timer.reset();
        }
    }

    pub fn explode(&mut self, show: &'static str) {
        self.destroyed = true;
        self.display = show;
        self.timer = Timer::new(2000)
    }

    pub fn is_expired(&self) -> bool {
        self.col == FRAME_COLS - 1 || self.timer.ready()
    }
}

impl Drawable for Bullet {
    fn draw(&self, frame: &mut Frame) {
        frame[self.row][self.col] = self.display.to_owned();
    }
}
