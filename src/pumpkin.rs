use rand::Rng;

use crate::{
    bullet::Bullet,
    frame::{Drawable, Frame, FRAME_COLS, FRAME_ROWS},
};

pub struct Pumpkins {
    items: Vec<Pumpkin>,
}

impl Pumpkins {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn update(&mut self) {
        // Populate with always 6 pumpkins
        while self.items.len() < 6 {
            // Generate spawn points excluding current pumpkin positions
            let prev_positions: Vec<usize> = self.items.iter().map(|item| item.row).collect();

            let free_positions: Vec<usize> = (2..FRAME_ROWS)
                .filter(|num| !prev_positions.contains(num))
                .collect();

            let spawn_row = free_positions[rand::thread_rng().gen_range(0..free_positions.len())];

            self.items.push(Pumpkin {
                row: spawn_row,
                col: FRAME_COLS / 2,
            });
        }
    }

    pub fn check_smash(&mut self, bullet: &mut Bullet, smashed: &mut bool) {
        let pumpkin_idx = self
            .items
            .iter()
            .position(|item| item.row == bullet.row && item.col == bullet.col);

        if let Some(idx) = pumpkin_idx {
            self.items.remove(idx);
            *smashed = true;
            bullet.explode("✴️");
        }
    }

    pub fn eaten(&mut self, row: usize, col: usize) {
        let pumpkin_idx = self
            .items
            .iter()
            .position(|item| item.row == row && item.col == col);

        if let Some(idx) = pumpkin_idx {
            self.items.remove(idx);
        }
    }
}

impl Drawable for Pumpkins {
    fn draw(&self, frame: &mut Frame) {
        for pumpkin in self.items.iter() {
            frame[pumpkin.row][pumpkin.col] = "🎃".to_owned();
        }
    }
}

pub struct Pumpkin {
    row: usize,
    col: usize,
}

impl Pumpkin {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}
