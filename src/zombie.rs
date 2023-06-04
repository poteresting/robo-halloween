use rand::Rng;

use crate::{
    bullet::Bullet,
    frame::{Drawable, Frame, FRAME_COLS, FRAME_ROWS},
    pumpkin::Pumpkins,
    timer::Timer,
};

pub struct Zombies {
    items: Vec<Zombie>,
    spawn_timer: Timer,
}

impl Zombies {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            spawn_timer: Timer::new(10000),
        }
    }

    pub fn update(&mut self) {
        for zombie in self.items.iter_mut() {
            zombie.update();
        }

        // Populate with max 4 zombies
        let delta = rand::thread_rng().gen_range(0..5);
        self.spawn_timer.decrease_timer(delta);

        if self.items.len() < 4 && self.spawn_timer.ready() {
            self.items.push(Zombie {
                row: rand::thread_rng().gen_range(2..FRAME_ROWS),
                col: FRAME_COLS - 2,
                timer: Timer::new(rand::thread_rng().gen_range(150..250)),
            });
            self.spawn_timer.reset();
        }
    }

    pub fn reached_end(&mut self) -> bool {
        self.items.iter().any(|zombie| zombie.col == 0)
    }

    pub fn check_kill(&mut self, bullet: &mut Bullet, killed: &mut bool) {
        let zombie_idx = self
            .items
            .iter()
            .position(|item| item.row == bullet.row && item.col <= bullet.col);

        if let Some(idx) = zombie_idx {
            self.items.remove(idx);
            *killed = true;
            bullet.explode("💥");
        }
    }

    pub fn eat_pumpkins(&mut self, pumpkins: &mut Pumpkins) {
        for zombie in self.items.iter_mut() {
            pumpkins.eaten(zombie.row, zombie.col);
        }
    }
}

impl Drawable for Zombies {
    fn draw(&self, frame: &mut Frame) {
        for zombie in self.items.iter() {
            frame[zombie.row][zombie.col] = "🧟".to_owned();
        }
    }
}

pub struct Zombie {
    row: usize,
    col: usize,
    timer: Timer,
}

impl Zombie {
    pub fn new(row: usize, col: usize, ms: i64) -> Self {
        Self {
            row,
            col,
            timer: Timer::new(ms),
        }
    }

    fn update(&mut self) {
        self.timer.decrease_timer(5);

        if self.col > 1 && self.timer.ready() {
            self.col -= 2;
            self.timer.reset();
        }
        // Emoji takes more than one character space
        if self.col == 1 {
            self.col = 0
        }
    }
}
