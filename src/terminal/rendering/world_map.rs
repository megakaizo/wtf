use std::io::{Stdout, Write};
use crossterm::{
    style::{
        SetForegroundColor, ResetColor, Print, SetAttribute, Attribute
    }, queue, cursor::MoveTo
}; 

use crate::world::types::World;


pub fn render_world(world: &World, stdout: &mut Stdout) {
    for idx in 0..world.map.len() {
        let coord = world.coord(idx);
        let entity = world.get(coord);
        let glyph = entity.cell.glyph();
        let attr = entity.cell.attribute();
        let color = world.get_color(coord);
        queue!(
            stdout,
            MoveTo(coord.x + 1, coord.y + 1),
            SetForegroundColor(color),
            SetAttribute(attr),
            Print(glyph),
            ResetColor,
            SetAttribute(Attribute::Reset),
        ).unwrap();
    }
    stdout.flush().unwrap();
}
