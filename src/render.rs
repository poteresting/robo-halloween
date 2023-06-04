use std::{
    io::{Stdout, Write},
    ptr,
};

use crossterm::{
    cursor::MoveTo,
    style::{Color, SetBackgroundColor},
    terminal::{Clear, ClearType},
    QueueableCommand,
};

use crate::frame::Frame;

pub fn render(curr_frame: &Frame, prev_frame: &Frame, output: &mut Stdout) {
    let is_first_render = ptr::eq(curr_frame, prev_frame);

    if is_first_render {
        output.queue(SetBackgroundColor(Color::DarkBlue)).unwrap();
        output.queue(Clear(ClearType::All)).unwrap();
        output.queue(SetBackgroundColor(Color::Black)).unwrap();
    }

    for (r_idx, row) in curr_frame.iter().enumerate() {
        for (c_idx, char) in row.iter().enumerate() {
            if *char != prev_frame[r_idx][c_idx] || is_first_render {
                output.queue(MoveTo(c_idx as u16, r_idx as u16)).unwrap();
                println!("{}", *char);
            }
        }
    }
    output.flush().unwrap();
}
