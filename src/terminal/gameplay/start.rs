use std::io::Stdout;

use crate::world::types::World;
use crate::terminal::rendering::{world_map::render_faction_view, border::draw_border};
use crate::terminal::gameplay::turns::turn_player;
use crate::ai::turns::turn_ai;


fn setup_ai_factions(world: &mut World, players_count: u16) -> Vec<u16> {
    let mut ai_factions: Vec<u16> = Vec::new();
    
    let total_ai_count = world.factions.len() as u16 - players_count;
    let mut total_added: u16 = 0;

    for faction_id in (0..world.factions.len()).rev() {
        if total_added < total_ai_count {
            ai_factions.push(faction_id as u16);
            total_added += 1;
        } else {
            break;
        }
    }
    ai_factions
} 


pub fn start_game_cycle(
    world: &mut World, 
    stdout: &mut Stdout, 
    offset_x: u16, offset_y: u16,
    ai_factions: Vec<u16>,
    players_count: u16,
    
) {
    loop {
        if players_count == 1 {
            let player_faction_id = world.factions[0].id;
            render_faction_view(world, player_faction_id, stdout, offset_x, offset_y);
        }
        if world.factions[world.current_move_faction_id as usize].is_dead {
                continue;
        } else if ai_factions.contains(&world.current_move_faction_id) {
            turn_ai(world);
        } else {
            turn_player(world, stdout, offset_x, offset_y);
        }
    }
}


pub fn run_gameplay(stdout: &mut Stdout) {
    // terminal rendering
    let offset_x: u16 = 20;
    let offset_y: u16 = 5;

    // world
    let width: u16 = 40;
    let height: u16 = 20;
    let forest_cov: f32 = 0.20;
    let water_cov: f32 = 0.15;
    let mountains_cov: f32 = 0.05;
    let total_factions: u16 = 4;
    let min_req_base_distance = 7;
    let energy_per_faction = 5;

    // ai
    let players_count: u16 = 2;
    
    let mut world = World::generate( 
        width, 
        height, 
        water_cov,
        forest_cov,
        mountains_cov,
        total_factions, 
        min_req_base_distance,
        energy_per_faction,
    );

    let ai_factions = setup_ai_factions(&mut world, players_count);

    draw_border(width, height, stdout, offset_x, offset_y);
    start_game_cycle(&mut world, stdout, offset_x, offset_y, ai_factions, players_count); 

}
