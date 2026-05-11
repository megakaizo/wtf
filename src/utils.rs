use rand::{rng, RngExt};

use crate::types::{Coord, Entity, World};


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


pub fn validate_action(coord: &Coord, world: &World, faction_id: &u16) -> bool {
    let x = coord.x;
    let y = coord.y;
    let neighbors = [
        world.get(Coord{x: x.saturating_sub(1), y: y}),
        world.get(Coord{x: x, y: y.saturating_sub(1)}),
        world.get(Coord{x: x + 1, y: y}),
        world.get(Coord{x: x, y: y + 1}),
    ];

    neighbors.iter().any(|entity| {
        entity.faction_id == Some(*faction_id)
    })
}
