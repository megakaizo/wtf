use rand::{rng, RngExt};

use crate::types::{CellType, Coord, Entity, World};


pub fn manhattan(c1: Coord, c2: Coord) -> u16 {
    let dx = c1.x.abs_diff(c2.x);
    let dy = c1.y.abs_diff(c2.y);
    dx + dy
} 


pub fn random_coord(width: u16, height: u16) -> Coord {
    let x = rng().random_range(0..width);
    let y = rng().random_range(0..height);
    Coord { x, y } 
}


pub fn place_entity(
    world: &mut World, 
    coord: Coord, 
    entity_type: CellType, 
    faction_id: Option<u16>
) {
    world.game_map
        [coord.y as usize]
        [coord.x as usize]
            = Entity { cell_type: entity_type, faction_id: faction_id }    
}
