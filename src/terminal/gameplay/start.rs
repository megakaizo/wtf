use std::io::Stdout;

use crate::world::types::World;
use crate::terminal::rendering::{world_map::{render_world, render_faction_view}, border::draw_border};
use crate::terminal::gameplay::turns::start_game_cycle;


pub fn run_gameplay(stdout: &mut Stdout) {
    let width: u16 = 40;
    let height: u16 = 20;
    let forest_cov: f32 = 0.20;
    let water_cov: f32 = 0.15;
    let mountains_cov: f32 = 0.05;
    let total_factions = 4;
    let min_req_base_distance = 7;
    let energy_per_faction = 5;

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
    draw_border(width, height, stdout);
    render_faction_view(&mut world, stdout);
    start_game_cycle(&mut world, stdout); 

}

