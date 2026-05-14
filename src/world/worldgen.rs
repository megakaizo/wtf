use std::collections::HashMap;

use rand::{rng, RngExt};

use crate::{
    world::{
        types::{
            Cell,
            Coord, 
            Entity, 
            FACTION_COLORS, 
            Faction,  
            World
        }, 
        map::{
            get_neighboors, 
            manhattan, 
            random_coord, 
            set_on_map
        }
    }
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
        factions.push(Faction { id: faction_id, color, is_dead: false, is_ai: is_ai, lands: HashMap::new() });
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


fn validate_near_base(coord: &Coord, world: &World, bases_coords: &[Coord]) -> bool {
    if world.get(*coord).cell == Cell::Base {
        return true;
    }

    for base_coord in bases_coords {
        if manhattan(*coord, *base_coord) < 5 {
            return true;
        }
    }
    return false
}

fn smooth_generation(world: &mut World, bases_coords: &[Coord]) {
    for _ in 0..3 {
        let mut new_map = world.game_map.clone();

        for y in 0..world.height {
            for x in 0..world.width {
                let coord = Coord{ x, y };
                let near_base = validate_near_base(&coord, world, bases_coords);
                if near_base {
                    continue;
                }

                let neighboors = get_neighboors(&coord, world, true);
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
                    set_on_map(&mut new_map, Cell::Mountain, &coord, None);
                    continue;
                } else if water_count >= 4 {
                    set_on_map(&mut new_map, Cell::Water, &coord, None);
                    continue;
                } else if forest_count >= 4 {
                    set_on_map(&mut new_map, Cell::Forest, &coord, None);
                    continue;
                }

            }
        }
        world.game_map = new_map;
    }
}


fn place_terrains(world: &mut World, bases_coords: &[Coord]) {
    for y in 0..world.height {
        for x in 0..world.width {
            let coord = Coord { x, y };
            let near_base = validate_near_base(&coord, world, bases_coords);
            if near_base {
                continue;
            }
            let r: f32 = rng().random();
            if r < world.mountains_cov {
                world.set(coord, Cell::Mountain, None);
            } else if r < world.water_cov + world.mountains_cov {
                world.set(coord, Cell::Water, None);
            } else if r < world.water_cov + world.forest_cov + world.mountains_cov {
                world.set(coord, Cell::Forest, None);    
            };
        }
    }
    smooth_generation(world, bases_coords);
}


pub fn init_world(
    total_players: &mut i32,
    width: u16, 
    height: u16, 
    water_cov: f32, 
    forest_cov: f32,
    mountains_cov: f32,
    total_factions: u16, 
    min_req_base_distance: u16,
    energy_per_faction: u16,
) -> World {
    let game_map: Vec<Vec<Entity>> = init_game_map(height, width);
    let factions: Vec<Faction> = init_factions(total_factions, total_players);
    let mut world: World = World { 
        width, 
        height, 
        forest_cov, 
        water_cov, 
        mountains_cov,
        game_map, 
        factions, 
        energy_per_faction, 
    };
    let bases_coords = place_bases(&mut world, min_req_base_distance);
    place_terrains(&mut world, &bases_coords);
    world
}
