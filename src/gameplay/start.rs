use std::io::Stdout;

use crate::world::worldgen::init_world;
use crate::rendering::{world_map::render_world, border::draw_border};
use crate::gameplay::turns::start_game_cycle;


pub fn run_gameplay(stdout: &mut Stdout) {
    let width: u16 = 80;
    let height: u16 = 40;
    let forest_cov: f32 = 0.20;
    let water_cov: f32 = 0.15;
    let mountains_cov: f32 = 0.05;
    let total_factions = 4;
    let min_req_base_distance = 7;
    let mut total_players = 1;
    let energy_per_faction = 5;

    let mut world = init_world(
        &mut total_players, 
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
    render_world(&world, stdout);
    start_game_cycle(&mut world, stdout); 

}

