use std::io::Stdout;

use crate::terminal::rendering::world_map::render_world;
use crate::terminal::toml_config::Config;
use crate::world::types::World;
use crate::terminal::rendering::{world_map::render_faction_view, border::draw_border};
use crate::terminal::gameplay::turns::turn_player;
use crate::ai::turns::turn_ai;


fn setup_ai_factions(world: &mut World, ai_count: u16) -> Vec<u16> {
    let mut ai_factions: Vec<u16> = Vec::new();

    let mut total_added: u16 = 0;

    for faction_id in (0..world.factions.len()).rev() {
        if total_added < ai_count {
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
            if world.fog_of_war {
                render_faction_view(world, player_faction_id, stdout, offset_x, offset_y);
            } else {
                render_world(world, stdout, offset_x, offset_y);
            }
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
    let config: Config = Config::load("wtf.toml".to_string());
    
    let mut world = World::generate(config.world);

    let ai_factions = setup_ai_factions(&mut world, config.ai.ai_count);
    let offset_x = config.terminal.offset_x;
    let offset_y = config.terminal.offset_y;
    
    let players_count = world.factions.len() - ai_factions.len();
    draw_border(
        world.width, 
        world.height, 
        stdout, 
        config.terminal.offset_x, 
        config.terminal.offset_y
    );
    start_game_cycle(&mut world, stdout, offset_x, offset_y, ai_factions, players_count as u16); 
}
