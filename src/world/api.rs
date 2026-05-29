use std::collections::HashMap;

use rand::{rng, RngExt};

use crate::world::types::{Coord, Entity, World, Cell};


impl World {
    pub fn manhattan(&self, c1: &Coord, c2: &Coord) -> u16 {
        let dx = c1.x.abs_diff(c2.x);
        let dy = c1.y.abs_diff(c2.y);
        dx + dy
    } 

    pub fn random_coord(&self, width: u16, height: u16) -> Coord {
        let x = rng().random_range(0..width);
        let y = rng().random_range(0..height);
        Coord { x, y } 
    }

    fn idx(&self, coord: Coord) -> usize {
        coord.y as usize * self.width as usize + coord.x as usize
    }

    pub fn coord(&self, idx: usize) -> Coord {
        Coord {
            x: (idx % self.width as usize) as u16,
            y: (idx / self.width as usize) as u16,
        }
    }

    pub fn get(
        &self, coord: Coord,
    ) -> &Entity {
        &self.map[self.idx(coord)]
    }

    pub fn set(
        &mut self,
        coord: Coord,
        cell: Cell,
        faction_id: Option<u16>,
    ) {
        let old_entity = *self.get(coord);
        let entity = Entity{cell, faction_id};
        let idx = self.idx(coord);
        self.map[idx] = entity;

        if let Some(faction_id) = faction_id {
            let faction = &mut self.factions[faction_id as usize];
            faction.lands.insert(coord, entity);
        }
        if let Some(old_faction_id) = old_entity.faction_id {
            if Some(old_faction_id) != faction_id {
                let old_faction = &mut self.factions[old_faction_id as usize];
                old_faction.lands.remove(&coord);
            }
        }
    }

    pub fn in_bounds(
        &self,
        coord: Coord,
    ) -> bool {

        coord.x < self.width
            && coord.y < self.height 
    }

    pub fn kill_faction(
        &mut self,
        faction_id: u16
    ) {
        let faction = &mut self.factions[faction_id as usize];
        faction.is_dead = true;
        faction.lands.clear();
    }

    pub fn get_neighbors_lands(&self, coord: &Coord, include_diagonal: bool, include_own: bool) -> HashMap<Coord, Entity> {
        let entity = self.get(*coord);
        let mut total_neighbors: HashMap<Coord, Entity> = HashMap::new();
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
            if self.in_bounds(n_coord) {
                let neighboor = *self.get(n_coord);
                if !include_own {
                    if entity.faction_id == neighboor.faction_id {
                        continue;
                    }
                }
                total_neighbors.insert(n_coord, neighboor);
            }
        }
    
        total_neighbors
    } 
}

