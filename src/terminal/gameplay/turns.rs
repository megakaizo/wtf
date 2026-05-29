use std::io::{Stdout};

use crossterm::{event::{Event, read, KeyCode, MouseEventKind, MouseButton}};

use crate::world::types::{World, Coord};
use crate::ai::turns::turn_ai;
use crate::terminal::rendering::world_map::{render_faction_view};


fn turn_player(world: &mut World, stdout: &mut Stdout, offset_x: u16, offset_y: u16,) {
    match read().unwrap() {
        Event::Mouse(event) => {
            match event.kind {
                MouseEventKind::Down(MouseButton::Left) => {
                    if event.column <= offset_x || event.row <= offset_y {
                        return;
                    }
                    let x = event.column - offset_x - 1;
                    let y = event.row - offset_y - 1;
                    let coord = Coord { x, y };
                    if world.in_bounds(coord) {
                        world.action(coord);
                        render_faction_view(world, stdout, offset_x, offset_y);
                    }
                }
                _ => {}
            }
        }
        Event::Key(event) => {
            match event.code {
                KeyCode::Esc => return,

                _ => {},
            }
        }
        _ => {} 
    }
}

pub fn start_game_cycle(world: &mut World, stdout: &mut Stdout, offset_x: u16, offset_y: u16, ai_factions: Vec<u16>) {
    loop {
        for faction_id in 0..world.factions.len()  {
            if world.factions[faction_id].is_dead {
                continue;
            } else if ai_factions.contains(&(faction_id as u16)) {
                turn_ai(world, stdout, offset_x, offset_y);
            } else {
                turn_player(world, stdout, offset_x, offset_y);
            }
        }
    }
}


