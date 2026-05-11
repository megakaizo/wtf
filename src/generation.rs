use rand::{rng, RngExt};

use crate::{
    types::{
        Cell,
        Coord, 
        Entity, 
        FACTION_COLORS, 
        Faction,  
        World
    }, 
    utils::{manhattan, random_coord}
};


fn init_game_map(height: u16, width: u16) -> Vec<Vec<Entity>> {
    let game_map: Vec<Vec<Entity>> = vec![
        vec![
            Entity{
                cell: Cell::Empty,
                faction_id: None,
            };
            width as usize
        ];
        height as usize
    ];
    game_map
}


fn init_factions(total_factions: u16, total_players: &mut i32) -> Vec<Faction> {
    let mut factions: Vec<Faction> = Vec::new();
    for faction_id in 0..total_factions {
        let color = FACTION_COLORS[faction_id as usize];
        let mut is_ai = false;
        if *total_players <= 0 {
            is_ai = true;
        }
        factions.push(Faction { id: faction_id, color, is_dead: false, is_ai: is_ai });
        *total_players -= 1;
    }
    factions
}


fn calculate_bases_coords(world: &World, min_req_base_distance: u16) -> (Vec<Coord>, Vec<u16>) {
    let mut bases_coords: Vec<Coord> = Vec::new();
    let mut bases_ids: Vec<u16> = Vec::new();

    for faction in &world.factions {
        loop {
            let new_coord = random_coord(world.width, world.height);
            let mut valid = true;
            for base_coord in &bases_coords {
                if manhattan(new_coord, *base_coord) <= min_req_base_distance {
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


fn place_bases(world: &mut World, min_req_base_distance: u16) -> Vec<Coord>{
    let (bases_coords, ids): (Vec<Coord>, Vec<u16>) = calculate_bases_coords(world, min_req_base_distance);
    for (base_coord, faction_id) in bases_coords.iter().zip(ids.iter()) {
        world.set(*base_coord, Cell::Base, Some(*faction_id));
    }
    bases_coords
}


fn place_terrains(world: &mut World, bases_coords: &[Coord]) {
    for y in 0..world.height {
        for x in 0..world.width {
            let coord = Coord { x, y };
            if world.get(coord).cell == Cell::Base {
                continue;
            }

            let mut near_base = false;
            for base_coord in bases_coords {
                if manhattan(coord, *base_coord) < 5 {
                    near_base = true;
                    break;
                }
            }
            if near_base {
                continue;
            }
            let r: f32 = rng().random();
            if r < world.water_cov {
                world.set(coord, Cell::Water, None);
            } else if r < world.water_cov + world.forest_cov {
                world.set(coord, Cell::Forest, None);    
            };
            continue;
        }
    }
}


pub fn init_world(
    total_players: &mut i32,
    width: u16, 
    height: u16, 
    water_cov: f32, 
    forest_cov: f32, 
    total_factions: u16, 
    min_req_base_distance: u16,
) -> World {
    let game_map: Vec<Vec<Entity>> = init_game_map(height, width);
    let factions: Vec<Faction> = init_factions(total_factions, total_players);
    let mut world: World = World { width, height, forest_cov, water_cov, game_map, factions };
    let bases_coords = place_bases(&mut world, min_req_base_distance);
    place_terrains(&mut world, &bases_coords);
    world
}
