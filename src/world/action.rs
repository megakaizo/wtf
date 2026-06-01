use std::collections::{VecDeque, HashSet, HashMap};

use crate::world::types::{Cell, Coord, Entity, World};


impl World { 
    fn check_supply(&self, coord: &Coord, faction_id: &u16) -> bool {
        let dirs = [
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
                if !self.in_bounds(next) {
                    continue;
                }
                let neighboor = self.get(next);
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

    pub fn has_supply(&self, entity: &Entity, coord: &Coord) -> bool {
        match entity.cell {
            Cell::Base => true,
            Cell::Territory => true,
            Cell::Fortress | Cell::Bridge => {
                self.check_supply(coord, &self.current_move_faction_id)
            },
            _ => false,
        }

    }

    pub fn is_valid_action(&mut self, coord: &Coord, check_neighbors_supply: bool) -> bool {
        if coord.x < 0 || coord.y < 0 {
            return false;
        }
        if !self.in_bounds(*coord) {
            return false;
        } 
        let old_entity = self.get(*coord);
        if let Some(old_faction_id) = old_entity.faction_id {
            if old_faction_id == self.current_move_faction_id {
                return false;
            }
        }
        let energy_cost = self.cost(old_entity.cell);
        let faction = &mut self.factions[self.current_move_faction_id as usize];

        if energy_cost > faction.current_move_energy {
            return false;
        }

        if check_neighbors_supply { 
            let neighbors = self.get_neighbors_lands(coord, false, true); 
            neighbors.iter().any(|(n_coord, entity)| {
                if entity.faction_id != Some(self.current_move_faction_id) {
                    return false;
                }
                self.has_supply(entity, n_coord)    
            })
        } else {
            true
        }
    }

    pub fn turn_next_faction(&mut self) { 
        self.factions[self.current_move_faction_id as usize]
            .current_move_energy = self.energy_per_faction;

        let mut faction_id = self.current_move_faction_id;
        loop {
            faction_id += 1;
            if faction_id >= self.factions.len() as u16 {
                faction_id = 0; 
            }
            
            if !self.factions[faction_id as usize].is_dead {
                break;
            }
        }
        self.current_move_faction_id = faction_id;
    }

    pub fn action(&mut self, coord: Coord) { 
        if self.is_valid_action(&coord, true) {
            let old_entity = *self.get(coord); 
            let energy_cost = self.cost(old_entity.cell);

            {
                let faction = &mut self.factions[self.current_move_faction_id as usize]; 
                faction.current_move_energy -= energy_cost;
            }

            let captured_cell = self.capture_result(old_entity.cell);
            if let Some(captured_entity_faction_id) = old_entity.faction_id 
                && old_entity.cell == Cell::Base {
                    self.kill_faction(captured_entity_faction_id);
            }
            self.set(coord, captured_cell, Some(self.current_move_faction_id));
            
            if self.factions[self.current_move_faction_id as usize].current_move_energy <= 0 {
                self.turn_next_faction();
            }
        }
    }

    pub fn get_visible_lands(&self, coord: &Coord) -> HashMap<Coord, Entity> {
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

                if nx < coord.x as i32 - self.vision(entity.cell)
                    || nx > coord.x as i32 + self.vision(entity.cell)
                    || ny < coord.y as i32 - self.vision(entity.cell)
                    || ny > coord.y as i32 + self.vision(entity.cell) {
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

