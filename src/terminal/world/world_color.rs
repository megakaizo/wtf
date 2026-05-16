use crossterm::style::Color;

use crate::world::types::{Coord, World};


impl World { 
    pub fn get_color(
        &self, coord: Coord,
    ) -> Color {
        let entity = self.get(coord);
        let mut color = entity.cell.color();
        if let Some(faction_id) = entity.faction_id {
            color = self.factions[faction_id as usize].color();
        };
    color
    }
}

