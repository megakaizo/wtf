use std::io::{Stdout};

use crossterm::{event::{Event, read, KeyCode, MouseEventKind, MouseButton}};

use crate::world::types::{Cell, Coord, World};
use crate::rendering::world_map::render_world;
use crate::gameplay::validation::validate_action;


fn make_action(
    coord: Coord, 
    faction_id: &u16, 
    energy_points: &mut u16, 
    world: &mut World
) {
    let old_entity = world.get(coord);
    if let Some(old_faction_id) = old_entity.faction_id {
        if old_faction_id == *faction_id {
            return
        };
    }
    if validate_action(&coord, world, faction_id) {
        let move_cost = old_entity.cell.cost();
        if move_cost > *energy_points {
            return
        };
        *energy_points -= move_cost;
        let captured_cell = old_entity.cell.capture_result();
        if let Some(captured_entity_faction_id) = old_entity.faction_id 
            && old_entity.cell == Cell::Base {
                world.kill_faction(captured_entity_faction_id);
        }
        world.set(coord, captured_cell, Some(*faction_id));
    }
}


fn turn_player(world: &mut World, faction_id: u16, stdout: &mut Stdout) {
    let mut energy_points = world.energy_per_faction;
    while energy_points > 0 {
        match read().unwrap() {
            Event::Mouse(event) => {
                match event.kind {
                    MouseEventKind::Down(MouseButton::Left) => {
                        if event.column == 0 || event.row == 0 {
                            continue;
                        }
                        let x = event.column - 1;
                        let y = event.row - 1;
                        let coord = Coord { x, y };
                        if world.in_bounds(coord) {
                            make_action(
                                coord, 
                                &faction_id, 
                                &mut energy_points,
                                world,
                            );
                            render_world(world, stdout);
                        }
                    }
                    _ => {}
                }
            }
            Event::Key(event) => {
                match event.code {
                    KeyCode::Esc => break,
                    _ => {},

                }
            }
            _ => {} 
        }
    }
}


pub fn start_game_cycle(world: &mut World, stdout: &mut Stdout) {
    let factions_len = world.factions.len();

    loop {
        for i in 0..factions_len {
            if world.factions[i].is_dead {
                continue;
            }
            turn_player(world, i as u16, stdout);
        }
    }
}


