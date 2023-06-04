use crate::{
    bullet::Bullet,
    frame::{Drawable, Frame, FRAME_ROWS},
    pumpkin::Pumpkins,
    zombie::Zombies,
};

pub struct Robot {
    row: usize,
    col: usize,
    bullets: Vec<Bullet>,
    display: &'static str,
    alive: bool,
}

impl Robot {
    pub fn new() -> Self {
        Self {
            row: FRAME_ROWS / 2 - 1,
            col: 0,
            bullets: Vec::new(),
            display: "🤖",
            alive: true,
        }
    }

    pub fn move_up(&mut self) {
        if self.row > 2 && self.alive {
            self.row -= 1;
        }
    }

    pub fn move_down(&mut self) {
        if self.row < FRAME_ROWS - 1 && self.alive {
            self.row += 1;
        }
    }

    pub fn shoot(&mut self) -> bool {
        if self.bullets.len() < 4 && self.alive {
            self.bullets.push(Bullet::new(self.row, self.col + 2));
            true
        } else {
            false
        }
    }

    pub fn smash_pumpkins(&mut self, pumpkins: &mut Pumpkins) -> bool {
        let mut smashed = false;
        for bullet in self.bullets.iter_mut() {
            pumpkins.check_smash(bullet, &mut smashed);
        }
        smashed
    }

    pub fn kill_zombies(&mut self, zombies: &mut Zombies) -> bool {
        let mut killed = false;
        for bullet in self.bullets.iter_mut() {
            zombies.check_kill(bullet, &mut killed);
        }
        killed
    }

    pub fn update(&mut self) {
        for bullet in self.bullets.iter_mut() {
            bullet.update();
        }
        self.bullets.retain(|bullet| !bullet.is_expired());
    }

    pub fn kill_player(&mut self, show: &'static str) {
        self.display = show;
        self.alive = false;
    }

    pub fn is_alive(&self) -> bool {
        self.alive
    }
}

impl Drawable for Robot {
    fn draw(&self, frame: &mut Frame) {
        frame[self.row][self.col] = self.display.to_owned();
        for bullet in self.bullets.iter() {
            bullet.draw(frame);
        }
    }
}
