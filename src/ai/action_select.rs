use std::collections::HashMap;

use crate::world::types::{Cell, Coord, Entity, World};


fn is_alive_faction(entity: &Entity, world: &World) -> bool {
    if let Some(faction_id) = entity.faction_id {
        if world.factions[faction_id as usize].is_dead {
            return false;
        } else {
            return true;
        };
    } else {
        return false;
    }
}


fn reward_cost(entity: &Entity, world: &World) -> i32 {
    match entity.cell {
        Cell::Base => {
            if is_alive_faction(entity, world) {
                return 50;
            } else {
                return 0;
            }
        },
        Cell::Bridge => {
            if is_alive_faction(entity, world) {
                return 20;
            } else {
                return 0;
            }
        },
        Cell::Territory => {
            if is_alive_faction(entity, world) {
                return 30;
            } else {
                return 0;
            }
        },
        Cell::Fortress => 0, 
        Cell::Forest => 4,
        Cell::Mountain => 3,
        Cell::Water => 2,
        Cell::Empty => 6,
    }
}


fn action_coeff(coord_action: &Coord, coord_reward: &Coord, action_cost: f32, reward_cost: f32, world: &World) -> f32 {
    let manhattan = world.manhattan(coord_action, coord_reward);
    reward_cost / (manhattan as f32 + action_cost) 
    
}


pub fn select_best_action(
    visible_lands: &HashMap<Coord, Entity>, 
    available_for_action_lands: &HashMap<Coord, Entity>,
    world: &World,
) -> Option<Coord> {
    let mut current_coeff: f32 = 0.0;
    let mut best_action_coord: Option<Coord> = None;

    for (coord_action, entity_action) in available_for_action_lands.iter() {
        let action_cost = world.cost(entity_action.cell);
        if action_cost > world.factions[world.current_move_faction_id as usize].current_move_energy {
            continue;
        }

        for (coord_visible, entity_visible) in visible_lands.iter() {
            let reward_cost = reward_cost(entity_visible, world);
            let action_coeff = action_coeff(
                coord_action, coord_visible, action_cost as f32, reward_cost as f32, world
            );

            if action_coeff > current_coeff {
                current_coeff = action_coeff;
                best_action_coord = Some(*coord_action);
            }       
        }
    }
    best_action_coord
}


