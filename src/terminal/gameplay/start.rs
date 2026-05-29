use std::io::Stdout;

use crate::world::types::World;
use crate::terminal::rendering::{world_map::{render_world, render_faction_view}, border::draw_border};
use crate::terminal::gameplay::turns::start_game_cycle;


fn setup_ai_factions(world: &mut World, total_ai_count: u16) -> Vec<u16> {
    let mut ai_factions: Vec<u16> = Vec::new();
    
    let total_added: u16 = 0;

    for faction_id in (0..world.factions.len()).rev() {
        if total_added < total_ai_count {
            ai_factions.push(faction_id as u16);
        }
    }
    ai_factions
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
    let total_factions = 4;
    let min_req_base_distance = 7;
    let energy_per_faction = 5;

    // ai
    let total_ai_count = 3;
    
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

    let ai_factions = setup_ai_factions(&mut world, total_ai_count);

    draw_border(width, height, stdout, offset_x, offset_y);
    render_faction_view(&mut world, stdout, offset_x, offset_y);
    start_game_cycle(&mut world, stdout, offset_x, offset_y, ai_factions); 

}

