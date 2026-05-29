use std::collections::HashMap;
use std::io::Stdout;
use std::thread;
use std::time::Duration;

use crate::world::types::{Coord, Entity, World};
use crate::ai::action_select::select_best_action;
use crate::terminal::rendering::world_map::render_faction_view;

pub fn turn_ai(world: &mut World, stdout: &mut Stdout, offset_x: u16, offset_y: u16) {
    let mut visible_lands: HashMap<Coord, Entity> = HashMap::new();
    let mut available_for_action_lands: HashMap<Coord, Entity> = HashMap::new();

    for coord in world.factions[world.current_move_faction_id as usize].lands.clone().keys() {
        visible_lands.extend(world.get_visible_lands(coord));
        available_for_action_lands.extend(world.get_lands_available_for_action(coord));
    }
    
    let best_action_coord = select_best_action(&visible_lands, &available_for_action_lands, world);
    if let Some(coord) = best_action_coord {
        world.action(coord);
        thread::sleep(Duration::from_secs_f32(0.5));
        render_faction_view(world, stdout, offset_x, offset_y);
    }
}
