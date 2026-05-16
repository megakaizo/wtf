use std::collections::HashMap;

use rand::{rng, RngExt};

use crate::world::types::{Coord, Entity, World, Cell, Faction};


impl World {
    fn create_map(height: u16, width: u16) -> Vec<Entity> {
        let game_map: Vec<Entity> = vec![
                Entity{
                cell: Cell::Empty,
                faction_id: None
            };
            width as usize * height as usize 
        ];
        game_map
    }

    fn create_factions(total_factions: u16, total_players: &mut i32, energy_per_faction: u16) -> Vec<Faction> {
        let mut factions: Vec<Faction> = Vec::new();
        for faction_id in 0..total_factions {
            let mut is_ai = false;
            if *total_players <= 0 {
                is_ai = true;
            }
            factions.push(
                Faction { 
                    id: faction_id, 
                    is_dead: false, 
                    is_ai: is_ai, 
                    lands: HashMap::new(), 
                    current_move_energy: energy_per_faction 
                }
            );
            *total_players -= 1;
        }
        factions
    }

    fn generate_base_positions(&self, min_req_base_distance: u16) -> (Vec<Coord>, Vec<u16>) {
        let mut bases_coords: Vec<Coord> = Vec::new();
        let mut bases_ids: Vec<u16> = Vec::new();

        for faction in &self.factions {
            loop {
                let new_coord = self.random_coord(self.width, self.height);
                let mut valid = true;
                for base_coord in &bases_coords {
                    if self.manhattan(new_coord, *base_coord) <= min_req_base_distance {
                        valid = false;
                        break;
                    }
                }
                if valid {
                    bases_coords.push(new_coord);
                    bases_ids.push(faction.id);
                    break;
                }
            }
        }
        (bases_coords, bases_ids)
    }
    
    fn generate_bases(&mut self, min_req_base_distance: u16) -> Vec<Coord>{
        let (bases_coords, ids): (Vec<Coord>, Vec<u16>) = self.generate_base_positions(min_req_base_distance);
        for (base_coord, faction_id) in bases_coords.iter().zip(ids.iter()) {
            self.set(*base_coord, Cell::Base, Some(*faction_id));
        }
        bases_coords
    }

    fn is_near_base(&self, coord: &Coord, bases_coords: &[Coord]) -> bool {
        if self.get(*coord).cell == Cell::Base {
            return true;
        }

        for base_coord in bases_coords {
            if self.manhattan(*coord, *base_coord) < 5 {
                return true;
            }
        }
        return false
    }

    fn smooth_map(&mut self, bases_coords: &[Coord]) {
        for _ in 0..3 {
            let mut new_map = self.map.clone();

            for idx in 0..self.map.len() {
                let coord = self.coord(idx);
                if self.is_near_base(&coord, bases_coords) {
                    continue;
                }

                let neighboors = self.get_neighbors(&coord, true);
                let mut forest_count = 0;
                let mut water_count = 0;
                let mut mountains_count = 0;
    
                for neighboor in neighboors {
                    match neighboor.cell {
                        Cell::Forest => forest_count += 1,
                        Cell::Water => water_count += 1,
                        Cell::Mountain => mountains_count += 1,
                        _ => {}
                    }
                }
                if mountains_count >= 3 {
                    new_map[idx] = Entity { cell: Cell::Mountain, faction_id: None };
                    continue;
                } else if water_count >= 4 {
                    new_map[idx] = Entity { cell: Cell::Water, faction_id: None };
                    continue;
                } else if forest_count >= 4 {
                    new_map[idx] = Entity { cell: Cell::Forest, faction_id: None };
                    continue;
                }
            }
            self.map = new_map;
        }
    }

    fn generate_terrains(
        &mut self, 
        bases_coords: &[Coord],
        water_cov: f32,
        forest_cov: f32,
        mountains_cov: f32,
    ) {
        for idx in 0..self.map.len() { 
            let coord = self.coord(idx);
            if self.is_near_base(&coord, bases_coords) {
                continue;
            }
            let r: f32 = rng().random();
            if r < mountains_cov {
                self.set(coord, Cell::Mountain, None);
            } else if r < water_cov + mountains_cov {
                self.set(coord, Cell::Water, None);
            } else if r < water_cov + forest_cov + mountains_cov {
                self.set(coord, Cell::Forest, None);    
            }; 
        }
        self.smooth_map(bases_coords);
    }

    pub fn generate(
        total_players: &mut i32,
        width: u16, 
        height: u16, 
        water_cov: f32, 
        forest_cov: f32,
        mountains_cov: f32,
        total_factions: u16, 
        min_req_base_distance: u16,
        energy_per_faction: u16,
    ) -> Self {
        let map: Vec<Entity> = Self::create_map(height, width);
        let factions: Vec<Faction> = Self::create_factions(total_factions, total_players, energy_per_faction);
        let mut world: World = World { 
            width, 
            height, 
            map, 
            factions, 
            energy_per_faction,
            current_move_faction_id: 0,
        };
        let bases_coords = world.generate_bases(min_req_base_distance);
        world.generate_terrains( 
            &bases_coords,
            water_cov,
            forest_cov,
            mountains_cov,
        );
        world
    }
}
