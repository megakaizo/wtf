use std::io::{Stdout, Write};
use crossterm::{
    style::{
        SetForegroundColor, ResetColor, Print
    }, queue, cursor::MoveTo
}; 

use crate::types::{Coord, World};


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
