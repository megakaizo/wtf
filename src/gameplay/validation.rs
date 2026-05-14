use std::collections::{VecDeque, HashSet};

use crate::world::{types::{Cell, Coord, World}, map::get_neighboors};


fn check_supply(coord: &Coord, world: &World, faction_id: &u16) -> bool {
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

            let next = Coord{x: nx as u16, y: ny as u16};
            if !world.in_bounds(next) {
                continue;
            }
            let neighboor = world.get(next);
            if neighboor.faction_id != Some(*faction_id) {
                continue;
            }

            match neighboor.cell {
                Cell::Base | Cell::Territory => {
                    return true;
                }
                Cell::Fortress | Cell::Bridge => {
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
            Cell::Fortress | Cell::Bridge => {
                check_supply(coord, world, faction_id)
            },
            _ => false,
        }
    })
}

