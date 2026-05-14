use std::io::{Stdout, Write};
use crossterm::{
    style::{
        SetForegroundColor, ResetColor, Print, SetAttribute, Attribute
    }, queue, cursor::MoveTo
}; 

use crate::world::types::{Coord, World};


pub fn render_world(world: &World, stdout: &mut Stdout) {
    for y in 0..world.height {
        for x in 0..world.width {
            let coord = Coord{x, y};
            let entity = world.get(coord);
            let glyph = entity.cell.glyph();
            let attr = entity.cell.attribute();
            let color = world.get_color(coord);
            queue!(
                stdout,
                MoveTo(x + 1, y + 1),
                SetForegroundColor(color),
                SetAttribute(attr),
                Print(glyph),
                ResetColor,
                SetAttribute(Attribute::Reset),
            ).unwrap();
        }   
    }
    stdout.flush().unwrap();
}
