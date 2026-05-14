use std::io::{Stdout, Write};
use crossterm::{
    style::Print, queue, cursor::MoveTo
}; 


pub fn draw_border(width: u16, height: u16, stdout: &mut Stdout) {
    let max_width = width + 1;
    let max_height = height + 1;

    for y in 0..=max_height {
        for x in 0..=max_width {
            let ch = match (x, y) {

                (0, 0) => '+',
                (w, 0) if w == max_width => '+',

                (0, h) if h == max_height => '+',

                (w, h)
                    if w == max_width
                    && h == max_height => '+',

                (0, _) => '|',
                (w, _) if w == max_width => '|',

                (_, 0) => '=',

                (_, h) if h == max_height => '=',

                _ => ' ',
            }; 
            queue!(
                stdout,
                MoveTo(x, y),
                Print(ch)
            ).unwrap();
        }
    }
    stdout.flush().unwrap();
}
