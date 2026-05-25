use std::collections::{VecDeque, HashSet, HashMap};

use crate::world::types::{Coord, World, Cell, Entity};


impl World {
    fn get_visible_lands(&self, coord: &Coord) -> HashMap<Coord, Entity> {
        let dirs = vec![
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

    pub fn get_faction_visible_lands(&mut self, faction_id: u16, include_own: bool) -> HashMap<Coord, Entity> {
        let own_lands = self.factions[faction_id as usize].lands.clone();
        let mut total_lands: HashMap<Coord, Entity> = HashMap::new(); 
        for coord in own_lands.keys() {
            let visible_lands = self.get_visible_lands(coord);
            total_lands.extend(visible_lands); 
        }
        if include_own {
            total_lands.extend(own_lands);
        }
        total_lands
    }

    pub fn get_lands_available_for_action() {
        // тут сделать логику расчета доступных земель для хода (мб юзать существующее action api /
        // делать стилизированную функцию по возможности)
    }
}
