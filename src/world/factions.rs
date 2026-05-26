use std::collections::{VecDeque, HashSet, HashMap};

use crate::world::types::{Coord, World, Entity};


impl World {
    fn get_visible_lands(&self, coord: &Coord) -> HashMap<Coord, Entity> {
        let dirs = [
            (-1,  0),
            ( 0, -1),
            ( 1,  0),
            ( 0,  1),
            (-1, -1),
            (1, -1),
            (-1, 1),
            (1, 1),
        ];
        let mut visible_entites: HashMap<Coord, Entity> = HashMap::new(); 
        let mut visited: HashSet<Coord> = HashSet::new();
        let mut queue = VecDeque::from([*coord]);
        
        let entity = self.get(*coord);

        while let Some(q_coord) = queue.pop_front() {
            for (dx, dy) in &dirs {
                let nx = q_coord.x as i32 + dx;
                let ny = q_coord.y as i32 + dy;
                if nx < 0 || ny < 0 {
                    continue;
                }

                if nx < coord.x as i32 - entity.cell.vision() 
                    || nx > coord.x as i32 + entity.cell.vision()
                    || ny < coord.y as i32 - entity.cell.vision()
                    || ny > coord.y as i32 + entity.cell.vision() {
                    continue;
                }
                let next = Coord{x: nx as u16, y: ny as u16};
                if !self.in_bounds(next) {
                    continue;
                }

                let next_entity = self.get(next);
                if next_entity.faction_id == entity.faction_id {
                    continue;
                }

                if !visited.contains(&next) {
                    visited.insert(next);
                    queue.push_back(next);
                    visible_entites.insert(next, *next_entity);
                }    
            }
        }
        visible_entites
    }
 
    pub fn get_lands_available_for_action(&mut self, coord: &Coord) -> HashMap<Coord, Entity> {
        let entity = self.get(*coord);
        let mut available_lands: HashMap<Coord, Entity> = HashMap::new();
        if let Some(entity_faction_id) = entity.faction_id {
            if entity_faction_id != self.current_move_faction_id {
                return available_lands;
            }
            if self.has_supply(entity, coord) {
                let neighbor_lands = self.get_neighbors_lands(coord, false, false);
                for (coord, entity) in neighbor_lands.iter() {
                    if self.is_valid_action(coord, false) {
                        available_lands.insert(*coord, *entity);
                    }
                } 
            }
            
        }
        available_lands 
    }
}
