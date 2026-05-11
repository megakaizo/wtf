use rand::{rng, RngExt};

use crate::types::{Coord};


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


