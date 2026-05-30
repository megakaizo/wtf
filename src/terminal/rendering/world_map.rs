use std::{collections::HashMap, io::{Stdout, Write}};
use crossterm::{
    style::{
        SetForegroundColor, ResetColor, Print, SetAttribute, Attribute, Color
    }, queue, cursor::MoveTo
}; 

use crate::world::types::{Coord, World, Entity};


fn set_entity_render(
    world: &World, 
    coord: &Coord, 
    entity: &Entity, 
    stdout: &mut Stdout,
    offset_x: u16,
    offset_y: u16,
) {
    let glyph = entity.cell.glyph();
    let attr = entity.cell.attribute();
    let color = world.get_color(*coord);
    queue!(
        stdout,
        MoveTo(coord.x + offset_x + 1, coord.y + offset_y + 1),
        SetForegroundColor(color),
        SetAttribute(attr),
        Print(glyph),
        ResetColor,
        SetAttribute(Attribute::Reset),
    ).unwrap();
}


fn set_fog_render(coord: &Coord, stdout: &mut Stdout, offset_x: u16, offset_y: u16) {
    queue!(
        stdout, 
        MoveTo(coord.x + offset_x + 1, coord.y + offset_y + 1),
        SetForegroundColor(Color::DarkGrey),
        Print('░'),
        ResetColor
    ).unwrap();
} 


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


pub fn render_faction_view(world: &mut World, faction_id: u16, stdout: &mut Stdout, offset_x: u16, offset_y: u16) {
    let mut faction_view = world.factions[faction_id as usize].lands.clone();

    for coord in world.factions[faction_id as usize].lands.keys() {
        faction_view.extend(world.get_visible_lands(coord));   
    } 

    for idx in 0..world.map.len() {
        let coord = world.coord(idx);
        if let Some(view_entity) = faction_view.get(&coord) {
            set_entity_render(world, &coord, view_entity, stdout, offset_x, offset_y); 
            continue;
        }
        set_fog_render(&coord, stdout, offset_x, offset_y);
    }
    stdout.flush().unwrap();
}
