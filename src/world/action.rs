use std::collections::{VecDeque, HashSet};

use crate::world::types::{Coord, World, Cell};


impl World { 
    fn check_supply(&self, coord: &Coord, faction_id: &u16) -> bool {
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

    fn is_valid_action(&self, coord: &Coord) -> bool {
        let neighbors = self.get_neighbors(coord, false); 
        neighbors.iter().any(|entity| {
            if entity.faction_id != Some(self.current_move_faction_id) {
                return false;
            }
            match entity.cell {
                Cell::Base => true,
                Cell::Territory => true,
                Cell::Fortress | Cell::Bridge => {
                    self.check_supply(coord, &self.current_move_faction_id)
                },
                _ => false,
            }
        })
    }

    fn turn_next_faction(&mut self) { 
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
        let old_entity = *self.get(coord);
        if let Some(old_faction_id) = old_entity.faction_id {
            if old_faction_id == self.current_move_faction_id {
                return
            };
        }

        if self.is_valid_action(&coord) {
            let energy_cost = old_entity.cell.cost();

            {
                let faction = &mut self.factions[self.current_move_faction_id as usize];

                if energy_cost > faction.current_move_energy {
                    return;
                }

                faction.current_move_energy -= energy_cost;
            }

            let captured_cell = old_entity.cell.capture_result();
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


}

