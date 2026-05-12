use std::io::{Stdout, Write};
use crossterm::{
    style::{
        SetForegroundColor, ResetColor, Print
    }, queue, cursor::MoveTo
}; 

use crate::types::{Coord, World};


pub fn show_menu(stdout: &mut Stdout) {
    queue!(
        stdout,
        MoveTo(10, 5),
        Print("War. Territory. Fortress."),

        MoveTo(10, 7),
        Print("[SPACE] -> START | [ESC] -> EXIT"),
    ).unwrap();
    stdout.flush().unwrap();
}


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


pub fn render_world(world: &World, stdout: &mut Stdout) {
    for y in 0..world.height {
        for x in 0..world.width {
            let coord = Coord{x, y};
            let glyph = world.get(coord).cell.glyph(); 
            let color = world.get_color(coord);    
            queue!(
                stdout,
                MoveTo(x + 1, y + 1),
                SetForegroundColor(color),
                Print(glyph),
                ResetColor,
            ).unwrap();
        }   
    }
    stdout.flush().unwrap();
}
