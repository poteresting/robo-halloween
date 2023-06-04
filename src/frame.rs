pub const FRAME_COLS: usize = 80; // corresponds x-axis
pub const FRAME_ROWS: usize = 23; // corresponds y-axis
pub type Frame = Vec<Vec<String>>;

pub fn frame() -> Frame {
    let mut rows = Vec::with_capacity(FRAME_ROWS);
    for _ in 0..FRAME_ROWS {
        let mut cols = Vec::with_capacity(FRAME_COLS);
        for _ in 0..FRAME_COLS {
            cols.push(" ".to_owned());
        }
        rows.push(cols);
    }
    rows
}

pub trait Drawable {
    fn draw(&self, frame: &mut Frame);
}
