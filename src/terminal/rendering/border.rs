use std::io::{Stdout, Write};
use crossterm::{
    style::Print, queue, cursor::MoveTo
}; 


pub fn draw_border(width: u16, height: u16, stdout: &mut Stdout, offset_x: u16, offset_y: u16) {
    let max_width = width + offset_x + 1;
    let max_height = height + offset_y + 1;

    for y in 0..=max_height {
        for x in 0..=max_width {
            let ch = match (x, y) {

                (w, h) if w == offset_x && h == offset_y => '╔',
                (w, h) if w == max_width && h == offset_y => '╗',

                (w, h) if w == offset_x && h == max_height => '╚',

                (w, h)
                    if w == max_width
                    && h == max_height => '╝',

                (w, h) if w == offset_x && h >= offset_y => '║',
                (w, h) if w == max_width && h >= offset_y => '║',

                (w, h) if w >= offset_x && h == offset_y => '═',

                (w, h) if w >= offset_x && h == max_height => '═',

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
