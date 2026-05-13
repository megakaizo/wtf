use std::collections::{VecDeque, HashSet};

use rand::{rng, RngExt};

use crate::types::{Cell, Coord, Entity, World};


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


pub fn set_on_map(map: &mut Vec<Vec<Entity>>, cell: Cell, coord: &Coord, faction_id: Option<u16>) {
    map[coord.y as usize][coord.x as usize] = Entity { cell, faction_id: faction_id };
}


pub fn get_neighboors(coord: &Coord, world: &World, include_diagonal: bool) -> Vec<Entity> {
    let mut total_neighbors: Vec<Entity> = Vec::new();
    let mut offsets = vec![
        (-1,  0),
        ( 0, -1),
        ( 1,  0),
        ( 0,  1),
    ];

    if include_diagonal {

        offsets.extend([
            (-1, -1),
            ( 1, -1),
            (-1,  1),
            ( 1,  1),
        ]);
    }
    
    for (dx, dy) in offsets {
        let n_x = coord.x as i32 + dx;
        let n_y = coord.y as i32 + dy;

        if n_x < 0 || n_y < 0 {
            continue;
        }

        let n_coord = Coord{x: n_x as u16, y: n_y as u16};
        if world.in_bounds(n_coord) {
            let neighboor = world.get(n_coord);
            total_neighbors.push(neighboor);
        }
    }
    
    total_neighbors
}


fn check_fortress_supply(coord: &Coord, world: &World, faction_id: &u16) -> bool {
    let dirs = vec![
        (1, 0),
        (0, 1),
        (-1, 0),
        (0, -1),
    ];

    let mut queue = VecDeque::from([*coord]);
    let mut visited = HashSet::from([*coord]);

    while let Some(coord) = queue.pop_front() {
        for (dx, dy) in &dirs {
            let nx = coord.x as i32 + dx;
            let ny = coord.y as i32 + dy;
            
            if nx < 0 || ny < 0 {
                continue;
            }
            if !world.in_bounds(coord) {
                continue;
            }
            let next = Coord{x: nx as u16, y: ny as u16};
            let neighboor = world.get(next);
            if neighboor.faction_id != Some(*faction_id) {
                continue;
            }

            match neighboor.cell {
                Cell::Base | Cell::Territory => {
                    return true;
                }
                Cell::Fortress => {
                    if visited.insert(next) {
                        queue.push_back(next);
                    }
                }
                _ => {}
            }
            
        }
    }
    false
}

pub fn validate_action(coord: &Coord, world: &World, faction_id: &u16) -> bool {
    let neighbors = get_neighboors(coord, world, false); 
    neighbors.iter().any(|entity| {
        if entity.faction_id != Some(*faction_id) {
            return false;
        }
        match entity.cell {
            Cell::Base => true,
            Cell::Territory => true,
            Cell::Fortress => {
                check_fortress_supply(coord, world, faction_id)
            },
            _ => false,
        }
    })
}

